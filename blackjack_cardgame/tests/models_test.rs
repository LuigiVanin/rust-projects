use blackjack_cardgame::models::*;

#[test]
fn creating_cards_test() {
    let card1 = Card::new(21, Naipe::Copas);
    assert_eq!(card1.is_err(), true);
    let card2 = Card::new(112, Naipe::Copas);
    assert_eq!(card2.is_err(), true);
    let card3 = Card::new(11, Naipe::Copas);
    assert_eq!(card3.is_err(), false);
    let card4 = Card::new(0, Naipe::Copas);
    assert_eq!(card4.is_err(), false);
    let card5 = Card::new(14, Naipe::Copas);
    assert_eq!(card5.is_err(), true);
}

#[test]
fn adding_card_to_player_test() {
    let mut player = Player::new("Teste Player".to_string(), 1);
    assert_eq!(player.hand.len(), 0);
    let card_to_pull = Card::new(11, Naipe::Ouros).unwrap();
    player.draw_card(card_to_pull);
    assert_eq!(player.hand.len(), 1);
}

#[test]
fn player_total_points_test() {
    let mut player = Player::new("Teste Player".to_string(), 1);
    player.draw_card(Card::new(11, Naipe::Ouros).unwrap());
    player.draw_card(Card::new(3, Naipe::Ouros).unwrap());
    player.draw_card(Card::new(3, Naipe::Ouros).unwrap());

    assert_eq!(player.total_points(), 11 + 3 + 3);
}

#[test]
fn getting_naipe_test() {
    assert!(Naipe::get_naipe(0).unwrap() == Naipe::Copas);
    assert!(Naipe::get_naipe(1).unwrap() == Naipe::Espadas);
    assert!(Naipe::get_naipe(2).unwrap() == Naipe::Ouros);
    assert!(Naipe::get_naipe(3).unwrap() == Naipe::Paus);
}

#[test]
fn getting_invalid_naipe_test() {
    let invalid_naipe = Naipe::get_naipe(5);

    assert_eq!(invalid_naipe.is_err(), true);
}
