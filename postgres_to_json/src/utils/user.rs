use crate::user::{
    builder::UserData,
    use_case::{CreateUserUseCase, UseCase},
};

pub fn read_user_data() -> UserData {
    let create_user = CreateUserUseCase::new();
    create_user.execute(UserData {
        username: "postgres".into(),
        password: "".into(),
        dbname: "".into(),
        host: "localhost".into(),
        port: "5432".into(),
    })
}
