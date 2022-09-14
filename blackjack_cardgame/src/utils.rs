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
