mod deck;
mod player;

use crate::deck::Deck;
use crate::player::Player;
use std::io;

fn main() {
    clearscreen::clear().expect("failed to clear screen");
    let mut input = String::new();

    let mut player = Player::create(1000);
    let mut dealer = Player::create(0);

    println!("Let's play black_jack\n\n");
    println!(
        "How much do you wanna bet? You have {}U$ left",
        player.money
    );
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: i32 = input.trim().parse().expect("Please enter a valid number");
    player.bet(num);

    //round start
    let mut deck = Deck::new();
    Deck::shuffle(&mut deck);

    player.add_hand(deck.deal().unwrap());
    player.add_hand(deck.deal().unwrap());
    dealer.add_hand(deck.deal().unwrap());

    // new buy loop
    loop {
        clearscreen::clear().expect("failed to clear screen");

        let dealer_sum = dealer.sum_value_cards();
        let player_sum = player.sum_value_cards();

        println!("Table's cards:");
        println!("Sums to {}", dealer_sum);
        dealer.print_cards();
        player.print_cards();
        println!("Sums to {}\n", player_sum);

        println!("Hit (h) or Stand (s)?:");

        let mut round_input = String::new();
        io::stdin()
            .read_line(&mut round_input)
            .expect("Failed to read line");

        let player_input = round_input.trim();

        if player_input == "h" {
            player.add_hand(deck.deal().unwrap());
        }
        if player_input == "s" {
            break;
        }
    }

    while dealer.sum_value_cards() < 17 {}

    //clear for next round
    clearscreen::clear().expect("failed to clear screen");
}
