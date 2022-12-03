use std::io::stdin;

pub fn read_line_clean(input: &mut String, default: &str) {
    input.clear();
    stdin()
        .read_line(input)
        .expect("Uepa! something went wrong!");
    *input = input.replace("\n", "");
    if input == "" {
        *input = default.to_string();
    }
}
