use crate::models::*;
use crate::utils::{colored_text, generate_rnd, read_line_clean, Style};
use core::panic;
use std::process::exit;

pub struct Game {
    pub deck: Vec<Card>,
    pub players: [Player; 2],
    pub round: u64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: Game::generate_deck(),
            players: [
                Player::new(Player::read_player_data(1), 1),
                Player::new(Player::read_player_data(2), 2),
            ],
            round: 0,
        }
    }

    fn turn(self: &mut Self) {
        for index in 0..self.players.len() {
            if self.players[index].stopped {
                continue;
            }
            println!(
                "-> {}, hora de jogar!\n[SPACE] to draw card; [s] to stop",
                self.players[index].name
            );
            match read_line_clean() {
                Ok(key) => match key.as_str() {
                    "s" => {
                        println!("{} Parou de jogar\n", self.players[index].name);
                        self.players[index].stopped = true;
                    }
                    " " => (),
                    _ => continue,
                },
                Err(_) => {
                    println!("\npassou a rodada!\n");
                    continue;
                }
            };
            match self.player_draw_card(index) {
                Ok(_) => (),
                Err(_) => continue,
            }

            self.players[index].print_hand();
            if Game::check_player_busted(&mut self.players[index]) {
                println!(
                    "{}",
                    colored_text(
                        format!("\n{} got busted!\n", self.players[index].name).as_str(),
                        31,
                        Style::Bold
                    )
                );
            }
        }
    }

    pub fn start(self: &mut Self) -> ! {
        self.shuffle_deck();
        loop {
            self.turn();
            if self.check_end_game() {
                self.declare_winner();
            }
        }
    }

    fn declare_winner(self: &Self) -> () {
        let mut winner: Option<Player> = None;
        if self.players[0].total_points() <= 21 {
            winner = Some(self.players[0].clone());
        }
        if (self.players[1].total_points() <= 21
            && self.players[0].total_points() < self.players[1].total_points())
            || winner.is_none()
        {
            winner = Some(self.players[1].clone());
        }

        if self.players[0].total_points() == self.players[1].total_points() {
            winner = None;
        }
        match winner {
            None => println!("Empate!!"),
            Some(player) => println!("\t==== {} Venceu!! ====\n", player.name),
        }
        self.end_game();
    }

    pub fn check_player_busted(player: &mut Player) -> bool {
        if player.total_points() > 21 {
            player.stopped = true;
            return true;
        }
        false
    }

    fn check_end_game(self: &Self) -> bool {
        for player in &self.players {
            if !player.stopped {
                return false;
            }
        }
        return true;
    }

    fn generate_deck() -> Vec<Card> {
        let card_amount = 52;
        let naipe_cards = 13;
        return (0..card_amount)
            .map(|z| {
                return Card::new(
                    (z % naipe_cards) + 1,
                    match Naipe::get_naipe(z / naipe_cards) {
                        Ok(value) => value,
                        Err(msg) => panic!("{}", msg),
                    },
                )
                .expect("Não se pode criar a cartas além de 12 :(");
            })
            .collect();
    }

    fn shuffle_deck(self: &mut Self) -> () {
        for i in 0..self.deck.len() {
            let random_idx = generate_rnd(0, self.deck.len().try_into().unwrap()) as usize;
            self.deck.swap(i, random_idx);
        }
    }

    fn pull_card(self: &mut Self) -> Option<Card> {
        self.deck.pop()
    }

    fn player_draw_card(self: &mut Self, turn: usize) -> Result<Card, ()> {
        if self.players[turn].stopped {
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

    fn end_game(self: &Self) -> () {
        println!(
            "{}",
            colored_text("\n\t==== Acabou o jogo ====\n", 32, Style::Bold)
        );
        exit(0)
    }
}
