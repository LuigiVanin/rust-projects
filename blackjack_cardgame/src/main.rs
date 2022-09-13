use core::panic;
use rand::Rng;
use std::io::stdin;

#[allow(dead_code)]
struct Player {
    name: String,
    hand: Vec<Card>,
}

impl Player {
    fn new(player_name: String) -> Player {
        Player {
            name: player_name,
            hand: vec![],
        }
    }

    pub fn read_player_data(id: i32) -> String {
        let mut input = String::new();
        println!("Insira um Nome para o jogador 1: [Player {}]", id);
        match stdin().read_line(&mut input) {
            Err(_) | Ok(1 | 0) => return format!("Player {}", id),
            Ok(_) => return input.replace("\n", ""),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Naipe {
    Copas = 0,
    Espadas = 1,
    Ouros = 2,
    Paus = 3,
}

impl Naipe {
    pub fn get_naipe(num: u8) -> Result<Naipe, String> {
        println!("{:?}", num);
        if num > 3 {
            return Err(String::from("Não temos mais de 4 naipes 🤔"));
        }
        return match num {
            0 => Ok(Naipe::Copas),
            1 => Ok(Naipe::Espadas),
            2 => Ok(Naipe::Ouros),
            3 => Ok(Naipe::Paus),
            _ => Err(String::from("Fuck 2")),
        };
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct Card {
    number: u8,
    naipe: Naipe,
}

struct Game {
    deck: Vec<Card>,
    players: [Player; 2],
}

impl Game {
    fn new() -> Game {
        Game {
            deck: Game::generate_deck(),
            players: [
                Player::new(Player::read_player_data(1)),
                Player::new(Player::read_player_data(2)),
            ],
        }
    }

    fn generate_deck() -> Vec<Card> {
        let card_amount = 52;
        let naipe_cards = 13;
        return (0..card_amount)
            .map(|z| {
                return Card {
                    number: z % naipe_cards,
                    naipe: match Naipe::get_naipe(z / naipe_cards) {
                        Ok(value) => value,
                        Err(msg) => panic!("{}", msg),
                    },
                };
            })
            .collect();
    }

    fn shuffle_deck(self: &mut Self) -> () {
        for i in 0..self.deck.len() {
            let random_idx = generate_rnd(0, 10) as usize;
            self.deck.swap(i, random_idx);
        }
    }

    fn pull_card(self: &mut Self) -> Option<Card> {
        self.deck.pop()
    }
}

fn generate_rnd(start: u8, end: u8) -> u8 {
    return rand::thread_rng().gen_range(start..end);
}
fn main() {
    let copas = Naipe::Copas;
    let paus = Naipe::Paus;

    let mut game = Game::new();
    game.shuffle_deck();
    println!("{:?}", game.deck);

    println!("{:?}", copas == paus);

    for i in game.players {
        println!("{:?}", i.name);
    }
}
