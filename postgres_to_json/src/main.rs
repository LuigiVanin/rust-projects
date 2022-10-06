use postgres::{Client, NoTls};

fn main() {
    let client = Client::connect(
        "host=localhost port=5432 user=postgres password=1337 dbname='voting-pool'",
        NoTls,
    );
    if let Err(_) = client {
        panic!("Ih deu ruim pai")
    }
    println!("Hello, world!");
}
