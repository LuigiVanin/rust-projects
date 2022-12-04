mod json;
mod utils;
use crate::utils::{sql::format_value_to_string, user::read_user_data};
use colored::Colorize;
use json::{builder::JsonBuilder, formatter::remove_leading_commas};
use postgres::{Client, NoTls, Row};
use std::{env, fs::File, io::Write};

fn main() -> () {
    let mut arguments = env::args();
    let root_path = arguments.next().unwrap();
    let target_path = arguments.next().get_or_insert("db.json".into()).clone();
    let user = read_user_data();
    let mut json_builder = JsonBuilder::init();
    println!("{:?}", user.get_db_url());
    let conn = Client::connect("postgresql://postgres:1337@localhost:5432/boardcamp", NoTls);
    let mut file = match File::create(&target_path) {
        Ok(it) => it,
        Err(_err) => return (),
    };
    println!(
        "{}",
        format!("\n\t├── {} → {}", target_path, "\n".normal().clear())
    );

    match conn {
        Ok(mut client) => {
            let table_names: Vec<Row> = client
                .query(
                    "
                    SELECT table_name 
                    FROM information_schema.tables 
                    WHERE table_schema = 'public';",
                    &[],
                )
                .unwrap();
            for table_row in table_names {
                let table_name: String = table_row.get(0);
                json_builder.create_list(table_name.clone());
                let rows = client
                    .query(
                        format!(
                            "
                    SELECT * FROM {}
                    ",
                            table_name
                        )
                        .as_str(),
                        &[],
                    )
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
            let mut json_text = json_builder.close();
            remove_leading_commas(&mut json_text);
            match file.write_all(json_text.as_bytes()) {
                Ok(_) => {
                    println!(
                        "{}",
                        format!(
                            "\n\t\t{} {}",
                            "  SQL to JSON - SUCEFULLY CONVERTED ✨  "
                                .on_green()
                                .white()
                                .bold(),
                            "\n".normal().clear()
                        )
                    );
                    client.close().expect("Uneable to close client!");
                }
                Err(_) => return (),
            };
        }
        Err(err) => {
            println!("error: {}", err);
            return ();
        }
    }
}
