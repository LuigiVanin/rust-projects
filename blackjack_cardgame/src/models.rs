use std::io::stdin;

#[derive(Clone)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub hand: Vec<Card>,
    pub stopped: bool,
}

impl Player {
    pub fn draw_card(self: &mut Self, card: Card) {
        self.hand.push(card)
    }

    pub fn new(player_name: String, id: i32) -> Player {
        Player {
            id,
            name: player_name,
            hand: vec![],
            stopped: false,
        }
    }

    pub fn print_hand(self: &Self) -> () {
        println!("{:?} -> {}", self.hand, self.total_points())
    }

    pub fn read_player_data(id: i32) -> String {
        let mut input = String::new();
        println!("Insira um Nome para o jogador 1: [Player {}]", id);
        match stdin().read_line(&mut input) {
            Err(_) | Ok(1 | 0) => return format!("Player {}", id),
            Ok(_) => return input.replace("\n", ""),
        }
    }

    pub fn total_points(self: &Self) -> u8 {
        self.hand
            .iter()
            .fold::<u8, _>(0, |acc, item| acc + item.number)
    }
}

// TODO: Implementar a trait de Debug para printar as cartas
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Naipe {
    Copas = 0,
    Espadas = 1,
    Ouros = 2,
    Paus = 3,
}

impl Naipe {
    pub fn get_naipe(num: u8) -> Result<Naipe, String> {
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
pub struct Card {
    pub number: u8,
    pub naipe: Naipe,
}

impl Card {
    pub fn new(number: u8, naipe: Naipe) -> Result<Card, ()> {
        if number > 13 {
            return Err(());
        }
        Ok(Card { naipe, number })
    }
}
