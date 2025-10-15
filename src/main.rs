mod deck;
mod player;

use crate::deck::Deck;
use crate::player::Player;
use std::io;
use std::{thread, time};

fn main() {
    clearscreen::clear().expect("failed to clear screen");

    let mut player = Player::create(1000);
    let mut dealer = Player::create(0);

    let mut lost = 0;

    println!("Welcome to Black Crab! The best Blackjack game in the world!\n\n");

    loop {
        let num: i32 = loop {
            println!(
                "How much do you wanna bet? You have {}U$ left",
                player.money
            );
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse() {
                Ok(num) => {
                    if num > player.money {
                        println!("You don't have enough money!");
                    } else {
                        break num;
                    }
                }
                Err(_) => println!("Please enter a valid number"),
            }
        };
        player.bet(num);

        //round start
        let mut deck = Deck::new();
        Deck::shuffle(&mut deck);

        player.empty_hand();
        dealer.empty_hand();

        player.add_hand(deck.deal().unwrap());
        player.add_hand(deck.deal().unwrap());
        dealer.add_hand(deck.deal().unwrap());
        dealer.add_hand(deck.deal().unwrap());

        // player's turn
        loop {
            clearscreen::clear().expect("failed to clear screen");

            let player_sum = player.sum_value_cards();

            println!("Table's cards:");
            dealer.print_dealer_cards();
            player.print_cards();
            println!("Sums to {}\n", player_sum);

            if player_sum > 21 {
                println!("Bust! You lose!");
                lost = 1;
                break;
            }
            if player_sum == 21 {
                println!("Blackjack! You win!");
                player.money += num + (num as f32 * 1.5) as i32;
            }

            println!("Hit or Stand? [h/s]:");

            let mut round_input = String::new();
            io::stdin()
                .read_line(&mut round_input)
                .expect("Failed to read line");

            let player_input = round_input.trim();

            match player_input {
                "h" => player.add_hand(deck.deal().unwrap()),
                "s" => break,
                _ => println!("Invalid input"),
            }
        }

        // dealer's turn
        if lost == 0 {
            loop {
                clearscreen::clear().expect("failed to clear screen");

                let dealer_sum = dealer.sum_value_cards();
                let player_sum = player.sum_value_cards();

                println!("Table's cards:");
                dealer.print_cards();
                player.print_cards();
                println!("Dealer sums to {}\n", dealer_sum);
                println!("Player sums to {}\n", player_sum);

                // The dealer must hit until their score is 17 or higher.
                if dealer_sum < 17 {
                    println!("Dealer hits");
                    dealer.add_hand(deck.deal().unwrap());
                    thread::sleep(time::Duration::from_millis(1000));
                } else {
                    // Compare the dealer's and player's scores to determine the winner.
                    match dealer_sum.cmp(&player_sum) {
                        std::cmp::Ordering::Greater => {
                            if dealer_sum > 21 {
                                println!("Dealer busts! You win!");
                                player.money += num * 2;
                            } else {
                                println!("Dealer wins!");
                            }
                        }
                        std::cmp::Ordering::Less => {
                            println!("You win!");
                            player.money += num * 2;
                        }
                        std::cmp::Ordering::Equal => {
                            println!("Push!");
                            player.money += num;
                        }
                    }
                    break;
                }
            }
        }

        if lost == 1 {
            println!("Not this time...");
            lost = 0;
        }

        println!("Play again? [Y/n]: ");
        let mut end = String::new();
        io::stdin()
            .read_line(&mut end)
            .expect("Failed to read line");

        if end == "n" {
            break;
        }

        //clear for next round
        clearscreen::clear().expect("failed to clear screen");
    }
}
