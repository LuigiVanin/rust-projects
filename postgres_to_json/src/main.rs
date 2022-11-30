use chrono::NaiveDate;
use postgres::{types::FromSql, Client, Column, NoTls, Row};
use std::{
    env,
    fmt::Display,
    fs::File,
    io::{stdin, Write},
};

enum SqlString {
    Value(String),
}

struct JsonBuilder {
    json: String,
    tab: u8,
}

impl JsonBuilder {
    pub fn init() -> JsonBuilder {
        JsonBuilder {
            json: "{\n".to_owned(),
            tab: 1,
        }
    }

    pub fn close(&mut self) -> String {
        self.json.push_str("\n}");
        return self.json.clone();
    }

    fn create_tabs(&self) -> String {
        let mut tabs = String::from("");
        for _ in 0..self.tab {
            tabs.push_str("\t");
        }

        tabs
    }

    pub fn create_item(&mut self, key: &str, value: String) {
        let tabs = self.create_tabs();
        self.json
            .push_str(format!("{}{},\n", tabs, convert_str_to_json_item(key, value)).as_str());
    }

    pub fn create_list(&mut self, list_name: String) {
        let tabs = self.create_tabs();
        self.json
            .push_str(format!("{}\"{}\": [\n", tabs, list_name).as_str());
        self.tab += 1;
    }

    pub fn create_unamed_obj(&mut self) {
        let tabs = self.create_tabs();
        self.json.push_str(format!("{}{{\n", tabs).as_str());
        self.tab += 1;
    }

    pub fn close_obj(&mut self) {
        self.tab -= 1;
        let tabs = self.create_tabs();
        self.json.push_str(format!("{}}},\n", tabs).as_str())
    }

    pub fn close_list(&mut self) {
        self.tab -= 1;
        let tabs = self.create_tabs();
        self.json.push_str(format!("{}],\n", tabs).as_str())
    }
}

impl SqlString {
    pub fn value(self: Self) -> String {
        match self {
            SqlString::Value(v) => v,
        }
    }

    pub fn with_quotes(self: Self) -> String {
        match self {
            SqlString::Value(v) => {
                if v == "null" {
                    return v;
                }
                format!("\"{}\"", v)
            }
        }
    }
}

#[derive(Debug)]
struct UserData {
    username: String,
    password: String,
    dbname: String,
    host: String,
    port: String,
}

impl UserData {
    fn get_db_url(self: Self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }
}

fn main() -> () {
    let mut arguments = env::args();
    let root_path = arguments.next().unwrap();
    let target_path = arguments.next().get_or_insert("./".into());
    let user = read_user_data();
    let mut json_builder = JsonBuilder::init();
    println!("{:?}", user.get_db_url());
    let conn = Client::connect("postgresql://postgres:1337@localhost:5432/boardcamp", NoTls);
    let mut file = match File::create("db.json") {
        Ok(it) => it,
        Err(err) => return (),
    };

    match conn {
        Ok(mut client) => {
            let table_names: Vec<Row> = client.query(
                "select table_name from information_schema.tables where table_schema = 'public';",
                &[],
            ).unwrap();
            for table_row in table_names {
                let table_name: String = table_row.get(0);
                json_builder.create_list(table_name.clone());
                let rows = client
                    .query(format!("select * from {}", table_name).as_str(), &[])
                    .unwrap();
                for row in &rows {
                    let columns = row.columns().clone();
                    json_builder.create_unamed_obj();
                    for col in columns {
                        // println!("name: {}, type: {}", col.name(), col.type_().name());
                        let value: Option<String> = format_value_to_string(row, col);
                        match value {
                            Some(strg) => json_builder.create_item(col.name(), strg.clone()),
                            None => {
                                println!("I dont know how to parse to shit")
                            }
                        }
                    }
                    json_builder.close_obj();
                }
                json_builder.close_list();
            }
            let json_text = json_builder.close();
            println!("{}", json_text);
            match file.write_all(json_text.as_bytes()) {
                Ok(_) => println!("Arquivo escrito com sucesso!"),
                Err(_) => return (),
            };
        }
        Err(err) => {
            println!("error: {}", err);
            return ();
        }
    }
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

fn convert_sql_result<'a, T: FromSql<'a> + Display>(row: &'a Row, column_name: &str) -> SqlString {
    let value: Option<T> = row.get(column_name);
    return match value {
        Some(v) => SqlString::Value(format!("{}", v)),
        None => SqlString::Value(format!("null")),
    };
}

fn convert_str_to_json_item(key: &str, value: String) -> String {
    format!("\"{}\": {}", key, value)
}

fn format_value_to_string(row: &Row, column: &Column) -> Option<String> {
    match column.type_().name() {
        "int4" => Some(convert_sql_result::<i32>(row, column.name()).value()),
        "text" | "varchar" => Some(convert_sql_result::<&str>(row, column.name()).with_quotes()),
        "real" => Some(convert_sql_result::<f32>(row, column.name()).value()),
        "bool" => Some(convert_sql_result::<bool>(row, column.name()).value()),
        "date" => Some(convert_sql_result::<NaiveDate>(row, column.name()).with_quotes()),
        _ => None,
    }
}
