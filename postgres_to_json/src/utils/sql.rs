use std::fmt::Display;

use chrono::NaiveDate;
use postgres::{types::FromSql, Client, Column, Row};

pub enum SqlString {
    Value(String),
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

pub fn convert_sql_result<'a, T: FromSql<'a> + Display>(
    row: &'a Row,
    column_name: &str,
) -> SqlString {
    let value: Option<T> = row.get(column_name);
    return match value {
        Some(v) => SqlString::Value(format!("{}", v)),
        None => SqlString::Value(format!("null")),
    };
}

pub fn format_value_to_string(row: &Row, column: &Column) -> Option<String> {
    match column.type_().name() {
        "int4" => Some(convert_sql_result::<i32>(row, column.name()).value()),
        "text" | "varchar" => Some(convert_sql_result::<&str>(row, column.name()).with_quotes()),
        "real" => Some(convert_sql_result::<f32>(row, column.name()).value()),
        "bool" => Some(convert_sql_result::<bool>(row, column.name()).value()),
        "date" => Some(convert_sql_result::<NaiveDate>(row, column.name()).with_quotes()),
        _ => None,
    }
}

pub fn query_table_names(client: &mut Client) -> Vec<Row> {
    client
        .query(
            "
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public';",
            &[],
        )
        .unwrap()
}

pub fn query_all_items(client: &mut Client, table_name: String) -> Vec<Row> {
    client
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
        .unwrap()
}
