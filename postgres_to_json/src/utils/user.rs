use crate::utils::io::read_line_clean;

#[derive(Debug)]
pub struct UserData {
    pub username: String,
    pub password: String,
    pub dbname: String,
    pub host: String,
    pub port: String,
}

impl UserData {
    pub fn get_db_url(self: Self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }
}

pub fn read_user_data() -> UserData {
    let mut user = UserData {
        username: "".into(),
        password: "".into(),
        dbname: "".into(),
        host: "".into(),
        port: "".into(),
    };
    println!("Input database username: [postgres]");
    read_line_clean(&mut user.username, "postgres");
    println!("Input database password:");
    read_line_clean(&mut user.password, "");
    println!("Input database database name:");
    read_line_clean(&mut user.dbname, "");
    println!("Input database host: [localhost]");
    read_line_clean(&mut user.host, "localhost");
    println!("Input database port: [5432]");
    read_line_clean(&mut user.port, "5432");

    return user;
}
