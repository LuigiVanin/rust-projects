use super::builder::{UserBuilder, UserData};

pub trait UseCase<Request, Response> {
    fn execute(&self, request: Request) -> Response;
}

pub struct CreateUserUseCase {}

impl CreateUserUseCase {
    pub fn new() -> Self {
        Self {}
    }
}

// implement usecase trait for CreateUserUseCase
impl UseCase<UserData, UserData> for CreateUserUseCase {
    fn execute(&self, default: UserData) -> UserData {
        let user_receiver = UserBuilder::new()
            .call("Input database host: [localhost]")
            .host(default.host)
            .call("Input database port: [5432]")
            .port(default.port)
            .call("Input database username: [postgres]")
            .username(default.username)
            .call("Input database password:")
            .password(default.password)
            .call("Input database name:")
            .dbname(default.dbname)
            .build();
        return user_receiver;
    }
}
