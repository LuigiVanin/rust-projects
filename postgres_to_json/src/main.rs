mod json;
mod utils;
use crate::utils::{sql::format_value_to_string, user::read_user_data};
use json::builder::JsonBuilder;
use postgres::{Client, NoTls, Row};
use std::{env, fs::File, io::Write};

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
        Err(_err) => return (),
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
