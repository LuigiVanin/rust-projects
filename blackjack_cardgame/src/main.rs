use crate::models::*;
mod models;
mod utils;

fn main() {
    let mut game = Game::new();
    game.shuffle_deck();
    println!("{:?}", game.deck);

    for i in game.players {
        println!("{:?}", i.name);
    }
}
