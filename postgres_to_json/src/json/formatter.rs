pub fn convert_str_to_json_item(key: &str, value: String) -> String {
    format!("\"{}\": {}", key, value)
}

#[allow(dead_code)]
pub fn remove_leading_comma(_json: String) -> String {
    todo!()
}
