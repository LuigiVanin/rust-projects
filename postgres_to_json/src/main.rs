use postgres::{types::FromSql, Client, Column, NoTls, Row};
use std::{
    borrow::Borrow,
    env,
    fmt::Display,
    fs::File,
    io::{stdin, Write},
};

#[derive(Debug)]
struct UserData {
    username: String,
    password: String,
    dbname: String,
    host: String,
    port: String,
}

enum Data {
    Int(i32),
    Str(String),
}

fn main() {
    let mut arguments = env::args();
    let root_path = arguments.next().unwrap();
    let target_path = arguments.next().get_or_insert("./".into());
    let user = read_user_data();
    println!("{:?}", user.get_db_url());
    let conn = Client::connect("postgresql://postgres:1337@localhost:5432/boardcamp", NoTls);
    match conn {
        Ok(mut client) => {
            let table_names: Vec<Row> = client.query(
                "select table_name from information_schema.tables where table_schema = 'public';",
                &[],
            ).unwrap();
            for table_row in table_names {
                let table_name: String = table_row.get(0);
                let rows = client
                    .query(format!("select * from {}", table_name).as_str(), &[])
                    .unwrap();
                println!("---------{:?}", table_name);
                for row in &rows {
                    let columns = row.columns().clone();
                    for j in columns {
                        println!("name: {}, type: {}", j.name(), j.type_().name());
                        let value: Option<String> = format_row_to_string(row, j);
                        match value {
                            Some(json) => {
                                println!("{}", json)
                            }
                            None => {
                                println!("I dont know how to parse to shit")
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            println!("error: {}", err);
            panic!("Ih deu ruim pai")
        }
    }
}

fn create_object(json_object: &mut String) -> String {
    format!("{{ {} }}", json_object)
}

fn read_line_clean(input: &mut String, default: &str) {
    input.clear();
    stdin()
        .read_line(input)
        .expect("Uepa! something went wrong!");
    *input = input.replace("\n", "");
    if input == "" {
        *input = default.to_string();
    }
}

fn read_user_data() -> UserData {
    let mut user = UserData {
        username: "".into(),
        password: "".into(),
        dbname: "".into(),
        host: "".into(),
        port: "".into(),
    };
    println!("Input database username: [postgres]");
    read_line_clean(&mut user.username, "postgres");
    println!("Input database password:");
    read_line_clean(&mut user.password, "");
    println!("Input database database name:");
    read_line_clean(&mut user.dbname, "");
    println!("Input database host: [localhost]");
    read_line_clean(&mut user.host, "localhost");
    println!("Input database port: [5432]");
    read_line_clean(&mut user.port, "5432");

    return user;
}

fn convert_sql_to_json<'a, T: FromSql<'a> + Display>(row: &'a Row, column_name: &str) -> String {
    let value: Option<T> = row.get(column_name);
    return match value {
        Some(v) => {
            format!("\"{}\": {}", column_name, v)
        }
        None => {
            format!("\"{}\": null", column_name)
        }
    };
}

fn format_row_to_string(row: &Row, column: &Column) -> Option<String> {
    match column.type_().name() {
        "int4" => Some(convert_sql_to_json::<i32>(row, column.name())),
        "text" => Some(convert_sql_to_json::<&str>(row, column.name())),
        _ => {
            println!("I dont know how to parse this shit!");
            None
        }
    }
}

impl UserData {
    fn get_db_url(self: Self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }
}
