mod json;
mod user;
mod utils;
use crate::utils::{
    logger::Logger,
    sql::{format_value_to_string, query_all_items, query_table_names},
    user::read_user_data,
};
use json::{builder::JsonBuilder, formatter::remove_leading_commas};
use postgres::{Client, NoTls, Row};
use std::{env, fs::File, io::Write};

fn create_obj_from_row(rows: &Vec<Row>, json_builder: &mut JsonBuilder) -> () {
    let console = Logger::new();
    for row in rows {
        json_builder.create_unamed_obj();
        for col in row.columns() {
            let value: Option<String> = format_value_to_string(row, col);
            match value {
                Some(strg) => json_builder.create_item(col.name(), strg.clone()),
                None => console.warning("Não sei como fazer a conversão deste item"),
            }
        }
        json_builder.close_obj();
    }
    json_builder.close_list();
}

fn main() -> () {
    let console = Logger::new();

    let mut arguments = env::args();
    arguments.next().unwrap();
    let target_path = arguments.next().get_or_insert("db.json".into()).clone();

    console.success("Iniciando Aplicação de conversão de postgres → JSON!");

    let user = read_user_data();
    let mut json_builder = JsonBuilder::init();
    let url = user.get_db_url();

    console.info(format!("arquivo para target: {}", url).as_str());
    let mut file = match File::create(&target_path) {
        Ok(it) => it,
        Err(_err) => return console.error("Erro ao criar arquivo"),
    };
    console.log("\n");
    console.info("Arquivo gerado com sucesso!");
    console.info(format!("url gerada: {}", url).as_str());
    console.info("Conectando ao banco dados...");
    let conn = Client::connect(url.as_str(), NoTls);

    match conn {
        Ok(mut client) => {
            let table_names: Vec<Row> = query_table_names(&mut client);
            console.log(format!("\n\t├── {}", target_path).as_str());
            for table_row in table_names {
                let table_name: String = table_row.get(0);
                console.log(format!("\t\t└─── Convertendo tabela: {} → json", table_name).as_str());
                json_builder.create_list(table_name.clone());
                let rows = query_all_items(&mut client, table_name);
                create_obj_from_row(&rows, &mut json_builder);
            }
            let mut json_text = json_builder.close().build();
            remove_leading_commas(&mut json_text);
            match file.write_all(json_text.as_bytes()) {
                Ok(_) => {
                    console.success("SQL to JSON - SUCEFULLY CONVERTED ✨");
                    client.close().expect("Uneable to close client!");
                }
                Err(_) => console.error("Um erro ocorreu ao converter"),
            };
        }
        Err(_) => {
            console.warning("Um erro ocorreu no moemnto de conectar ao banco de dados, tente mudar as credenciais");
            console.error("Um erro ocorreu ao conectar ao banco de dados");
            return ();
        }
    }
}
