mod deck;
mod player;

use crate::deck::Deck;
use crate::player::Player;
use std::io;

fn main() {
    
    let mut dealer = Player {
        hand: Vec::new(),
        money: 10000,
    };
    let mut player = Player {
        hand: Vec::new(),
        money: 1000,
    };

    clearscreen::clear().expect("failed to clear screen");
    println!("Let's play black_jack\n\n");
    
    let mut input = String::new();

    let mut playing = true;
    while playing {

        println!("How much do you wanna bet? You have {}U$ left", player.money);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num: i32 = input.trim().parse().expect("Please enter a valid number");

        player.bet(num);
        println!("Você apostou {}U$", num);

        let mut deck = Deck::new();
        Deck::shuffle(&mut deck);

        let mut round = 1;
        while round > 0 {
            clearscreen::clear().expect("failed to clear screen");

            if let Some(card) = deck.deal() {
                dealer.hand.push(card);
            }
            if let Some(card) = deck.deal() {
                player.hand.push(card);
            }

            println!("Table's cards:");
            dealer.print_cards();
            let dealer_sum = dealer.sum_value_cards();
            println!("Sums to {}", dealer_sum);


            player.print_cards();
            println!("{}", player.sum_value_cards());
            println!("You have {}U$", player.money);
            let hand_value = player.sum_value_cards();

            if round == 1 && hand_value == 21{
                round = 0;
            }

            round = 0;

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
