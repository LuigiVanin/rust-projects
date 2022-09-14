use crate::game::Game;
mod game;
mod models;
mod utils;

fn main() {
    let mut game = Game::new();
    game.start();
}
