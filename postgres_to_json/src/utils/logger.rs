use colored::Colorize;

pub struct Logger();

impl Logger {
    pub fn new() -> Logger {
        Logger()
    }

    pub fn error(&self, message: &str) {
        println!(
            "{}",
            format!(
                "\n\t\t{} {}",
                format!("  {}  ", message).on_red().white().bold(),
                "\n".normal().clear()
            )
        );
    }

    pub fn success(&self, message: &str) {
        println!(
            "{}",
            format!(
                "\n\t\t{} {}",
                format!("  {}  ", message).on_green().white().bold(),
                "\n".normal().clear()
            )
        );
    }

    pub fn info(&self, message: &str) {
        print!(
            "{}",
            format!(
                "{}{} {}",
                "Info: ".blue(),
                message.blue(),
                "\n".normal().clear()
            )
        );
    }

    pub fn warning(&self, message: &str) {
        print!(
            "{}",
            format!(
                "\n{}{} {}",
                "Warn: ".yellow(),
                message.yellow(),
                "\n".normal().clear()
            )
        );
    }

    pub fn log(&self, message: &str) {
        println!("{}", message);
    }
}
