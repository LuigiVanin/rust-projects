use std::collections::HashMap;

use blackjack_cardgame::{
    game::*,
    models::{Card, Naipe},
};

#[test]
fn adding_players_to_game() {
    let g = Game::new();
    assert_eq!(g.players.len(), 2);
    assert_eq!(g.deck.len(), 52);

    let mut hashmap = HashMap::new();

    g.deck.iter().for_each(|card: &Card| {
        let naipe = card.naipe;
        hashmap.entry(naipe).and_modify(|e| *e += 1).or_insert(1);
    });
    hashmap.keys().for_each(|n: &Naipe| {
        let r = hashmap.get(n);
        match r {
            Some(v) => {
                assert_eq!(v, &13)
            }
            None => fail(),
        }
    })
}

fn fail() {
    assert!(false);
}
