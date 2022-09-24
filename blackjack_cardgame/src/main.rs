use crate::game::Game;
use blackjack_cardgame::utils::{colored_text, Style};
mod game;
mod models;
mod utils;

fn main() {
    println!(
        "{}",
        colored_text("\n\t==== BlackJack Game ====\n", 32, Style::Bold)
    );
    let mut game = Game::new();

    println!(
        "{}",
        colored_text("\n\t==== Start Game ====\n", 32, Style::Bold)
    );

    game.start();
}
