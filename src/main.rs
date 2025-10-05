mod deck;
mod player;

use crate::deck::Deck;
use crate::player::Player;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let playing = Arc::new(AtomicBool::new(true));
    let p = playing.clone();

    ctrlc::set_handler(move || {
        println!("
 exiting black_jack... for now.");
        p.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

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

    while playing.load(Ordering::SeqCst) {
        if !playing.load(Ordering::SeqCst) {
            break;
        }

        player.print_cards();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}