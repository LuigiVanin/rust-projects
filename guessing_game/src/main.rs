use rand::Rng;
use std::io::stdin;

#[allow(dead_code)]
enum Style {
    Bold,
    Normal,
    Dimmed,
}

fn read_line_clean(input: &mut String) {
    input.clear();
    stdin()
        .read_line(input)
        .expect("Uepa! something went wrong!");
    *input = input.replace("\n", "");
}

fn colored_text(text: &str, code: i32, style: Style) -> String {
    let font_style = match style {
        Style::Bold => "1",
        Style::Normal => "0",
        Style::Dimmed => "2",
    };
    format!("\x1b[{};{}m{}\x1b[0m", font_style, code, text,)
}

fn main() {
    println!(
        "{}",
        colored_text("Hello, this is my guessing game!✨", 32, Style::Bold)
    );
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(0..100);
    let mut input = String::new();
    let mut guess: u32;
    loop {
        read_line_clean(&mut input);
        guess = match input.clone().parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                println!("Por favor, insira um valor válido!");
                continue;
            }
        };
        if guess > number {
            println!("guess a little lower");
        } else if guess < number {
            println!("guess a little higher");
        } else {
            println!("Yep, u are right, it is {:?}", number);
            break;
        }
    }

    println!("End of the program");
}
