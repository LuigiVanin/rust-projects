use crate::utils::io::read_line_clean;

#[derive(Clone)]
pub struct UserData {
    pub username: String,
    pub password: String,
    pub dbname: String,
    pub host: String,
    pub port: String,
}

impl UserData {
    pub fn get_db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.dbname
        )
    }
}

// create struct FOR user builder
pub struct UserBuilder {
    user: UserData,
}

// implementation of user builder using user input as values, use read_line_clean() to get user input
impl UserBuilder {
    pub fn new() -> Self {
        Self {
            user: UserData {
                username: "".into(),
                password: "".into(),
                dbname: "".into(),
                host: "".into(),
                port: "".into(),
            },
        }
    }

    pub fn username(mut self, default: String) -> Self {
        read_line_clean(&mut self.user.username, default.as_str());
        self
    }

    pub fn password(mut self, default: String) -> Self {
        read_line_clean(&mut self.user.password, default.as_str());
        self
    }

    pub fn dbname(mut self, default: String) -> Self {
        read_line_clean(&mut self.user.dbname, default.as_str());
        self
    }

    pub fn host(mut self, default: String) -> Self {
        read_line_clean(&mut self.user.host, default.as_str());
        self
    }

    pub fn port(mut self, default: String) -> Self {
        read_line_clean(&mut self.user.port, default.as_str());
        self
    }

    pub fn build(self) -> UserData {
        self.user
    }

    pub fn call(self, message: &str) -> Self {
        println!("{}", message);
        self
    }
}
