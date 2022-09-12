#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Naipe {
    Copas,
    Espadas,
    Paus,
    Ouros,
}

impl Naipe {
    pub fn get_naipe(num: u8) -> Result<Naipe, String> {
        println!("{:?}", num);
        if num > 3 {
            return Err(String::from("Fuck"));
        }
        return match num {
            0 => Ok(Naipe::Copas),
            1 => Ok(Naipe::Espadas),
            2 => Ok(Naipe::Ouros),
            3 => Ok(Naipe::Paus),
            _ => Err(String::from("Fuck 2")),
        };
    }

    // pub fn to_num(num: u8) -> Result<Naipe, String> {
    //     match num {
    //         0 => Ok(Naipe::Copas),
    //         1 => Ok(Naipe::Espadas),
    //         2 => Ok(Naipe::Ouros),
    //         3 => Ok(Naipe::Paus),
    //         _ => Err(String::from("Fuck 2")),
    //     };
    // }
}

#[derive(Debug)]
struct Card {
    number: u8,
    naipe: Naipe,
}

struct Game {
    deck: Vec<Card>,
    players: [String; 2],
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: Game::generate_deck(),
            players: ["Player1".to_string(), "Player2".to_string()],
        }
    }

    fn generate_deck() -> Vec<Card> {
        let card_amount = 52;
        let naipe_cards = 13;
        return (0..card_amount)
            .map(|z| {
                return Card {
                    number: z % naipe_cards,
                    naipe: Naipe::get_naipe(z / naipe_cards).expect("Erradoooooo!"),
                };
            })
            .collect();
    }

    fn shuffle_deck(self: &Self) -> ! {}
}

fn main() {
    let copas = Naipe::Copas;
    let paus = Naipe::Paus;

    let game = Game::new();
    println!("{:?}", game.deck);

    println!("{:?}", copas == paus);
}
