use crate::models::*;
use crate::utils::generate_rnd;
use crate::utils::read_line_clean;
use core::panic;
use std::convert::TryInto;
use std::process::exit;

pub struct Game {
    pub deck: Vec<Card>,
    pub players: [Player; 2],
    pub round: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: Game::generate_deck(),
            players: [
                Player::new(Player::read_player_data(0), 0),
                Player::new(Player::read_player_data(1), 1),
            ],
            round: 0,
        }
    }

    fn turn(self: &mut Self) {
        for index in 0..self.players.len() {
            if self.players[index].is_burn {
                continue;
            }
            println!(
                "{}, hora de jogar!\n[SPACE] to draw card; [s] to stop",
                self.players[index].name
            );
            match read_line_clean() {
                Ok(key) => match key.as_str() {
                    "s" => self.players[index].is_burn = true,
                    " " => (),
                    _ => continue,
                },
                Err(_) => {
                    println!("passou a rodada!")
                }
            };
            match self.player_draw_card(index) {
                Ok(_) => (),
                Err(_) => continue,
            }
        }
    }

    pub fn start(self: &mut Self) -> ! {
        self.shuffle_deck();
        loop {
            self.turn();
        }
    }

    pub fn generate_deck() -> Vec<Card> {
        let card_amount = 52;
        let naipe_cards = 13;
        return (0..card_amount)
            .map(|z| {
                return Card::new(
                    z % naipe_cards,
                    match Naipe::get_naipe(z / naipe_cards) {
                        Ok(value) => value,
                        Err(msg) => panic!("{}", msg),
                    },
                )
                .expect("Não se pode criar a cartas além de 12 :(");
            })
            .collect();
    }

    pub fn shuffle_deck(self: &mut Self) -> () {
        for i in 0..self.deck.len() {
            let random_idx = generate_rnd(0, self.deck.len().try_into().unwrap()) as usize;
            self.deck.swap(i, random_idx);
        }
    }

    pub fn pull_card(self: &mut Self) -> Option<Card> {
        self.deck.pop()
    }

    pub fn player_draw_card(self: &mut Self, turn: usize) -> Result<Card, ()> {
        if self.players[self.round].is_burn {
            return Err(());
        }
        match self.pull_card() {
            Some(card) => {
                self.players[turn].draw_card(card);
                return Ok(card);
            }
            None => Err(self.end_game()),
        }
    }

    pub fn end_game(self: &Self) -> () {
        println!("Acabou o jogo");
        exit(0)
    }
}
