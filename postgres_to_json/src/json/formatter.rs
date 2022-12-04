use unicode_segmentation::UnicodeSegmentation;

pub fn convert_str_to_json_item(key: &str, value: String) -> String {
    format!("\"{}\": {}", key, value)
}

pub fn remove_leading_commas(json: &mut String) -> () {
    let mut last_str = 0;
    let mut json_vec = json.graphemes(true).collect::<Vec<&str>>();

    for idx in 0..json_vec.len() {
        if json_vec[last_str] == "," && ["]", "}"].contains(&json_vec[idx]) {
            json_vec[last_str] = ""
        }

        match json_vec[idx] {
            " " | "\n" | "\t" => continue,
            _ => last_str = idx,
        }
    }

    *json = convert_str_vec_to_string(json_vec);
}

fn convert_str_vec_to_string(list_str: Vec<&str>) -> String {
    let mut text = String::from("");
    for j in list_str {
        text.push_str(j);
    }
    text
}
