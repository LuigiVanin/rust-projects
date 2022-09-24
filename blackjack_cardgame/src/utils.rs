use rand::Rng;
use std::io::stdin;

pub fn generate_rnd(start: u8, end: u8) -> u8 {
    return rand::thread_rng().gen_range(start..end);
}

pub fn read_line_clean() -> Result<String, ()> {
    let mut input = String::new();
    input.clear();
    match stdin().read_line(&mut input) {
        Ok(1) | Ok(0) | Err(_) => Err(()),
        Ok(_) => Ok(input.replace("\n", "")),
    }
}

#[allow(dead_code)]
pub enum Style {
    Bold,
    Normal,
    Dimmed,
}

#[allow(dead_code)]
pub fn colored_text(text: &str, code: i32, style: Style) -> String {
    let font_style = match style {
        Style::Bold => "1",
        Style::Normal => "0",
        Style::Dimmed => "2",
    };
    format!("\x1b[{};{}m{}\x1b[0m", font_style, code, text,)
}
