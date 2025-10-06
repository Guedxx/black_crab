mod deck;
mod player;

use crate::deck::Deck;
use crate::player::Player;
use std::io;
use std::sync::Arc;

fn main() {
    
    let mut _dealer = Player {
        hand: Vec::new(),
        money: 10000,
    };
    let mut player = Player {
        hand: Vec::new(),
        money: 1000,
    };

    let mut deck = Deck::new();
    Deck::shuffle(&mut deck);

    if let Some(card) = deck.deal() {
        player.hand.push(card);
    }
    if let Some(card) = deck.deal() {
        player.hand.push(card);
    }

    clearscreen::clear().expect("failed to clear screen");
    println!("Let's play black_jack");
    
    let mut playing = true;
    while playing {
        
        player.print_cards();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim_end() == "c" {
            playing = false;
        }
    }
}
