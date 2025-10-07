mod deck;
mod player;

use crate::deck::Deck;
use crate::player::Player;
use std::io;

fn main() {
    clearscreen::clear().expect("failed to clear screen");
    println!("Let's play black_jack\n\n");

    let mut dealer = Player {
        hand: Vec::new(),
        money: 10000,
    };
    let mut player = Player {
        hand: Vec::new(),
        money: 1000,
    };

    let mut input = String::new();
    let mut playing = true;

    while playing {
        println!(
            "How much do you wanna bet? You have {}U$ left",
            player.money
        );

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num: i32 = input.trim().parse().expect("Please enter a valid number");

        player.bet(num);
        println!("Você apostou {}U$", num);

        let mut deck = Deck::new();
        Deck::shuffle(&mut deck);

        let mut round = 1;

        if let Some(card) = deck.deal() {
            player.hand.push(card);
        }

        let mut player_action = String::new();
        while round > 0 {
            clearscreen::clear().expect("failed to clear screen");

            if dealer.sum_value_cards() < 15 {
                if let Some(card) = deck.deal() {
                    dealer.hand.push(card);
                }
            }

            let mut player_input = player_action.trim();
            if player_input == "h" {
                if let Some(card) = deck.deal() {
                    player.hand.push(card);
                }
            }

            println!("Table's cards:");
            dealer.print_cards();
            let dealer_sum = dealer.sum_value_cards();
            println!("Sums to {}", dealer_sum);

            player.print_cards();
            let hand_value = player.sum_value_cards();
            println!("Sums to {}\n", hand_value);
            println!("You have {}U$", player.money);

            println!("Hit (h) or Stand (s)?:");
            io::stdin()
                .read_line(&mut player_action)
                .expect("Failed to read line");

            if round == 2 && hand_value == 21 {
                round = 0;
            }

            if player.sum_value_cards() > 21 {
                println!("You loose!")
            }

            if player.sum_value_cards() == 21 {
                println!("You win! いい！")
            }
        }

        // Taking input and a end check
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim_end() == "c" {
            playing = false;
        }

        //clear for next round
        clearscreen::clear().expect("failed to clear screen");
    }
}
