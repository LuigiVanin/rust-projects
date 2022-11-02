use postgres::{Client, NoTls, Row};
use std::io::stdin;

#[derive(Debug)]
struct UserData {
    username: String,
    password: String,
    dbname: String,
    host: String,
    port: String,
}

fn main() {
    let user = read_user_data();
    println!("{:?}", user.get_db_url());
    let conn = Client::connect("postgresql://postgres:1337@localhost:5432/boardcamp", NoTls);
    match conn {
        Ok(mut client) => {
            let table_names: Vec<Row> = client.query(
                "select table_name from information_schema.tables where table_schema = 'public';",
                &[],
            ).unwrap();
            for row in table_names {
                let table_name: String = row.get(0);
                println!("{:?}", table_name);
            }
        }
        Err(err) => {
            println!("error: {}", err);
            panic!("Ih deu ruim pai")
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

impl UserData {
    fn get_db_url(self: Self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }
}
