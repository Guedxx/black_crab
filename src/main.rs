mod deck;
mod player;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::player::Player;
use crate::deck::Deck;

fn main() {

    
    let playing = Arc::new(AtomicBool::new(true));
    let p = playing.clone();

    ctrlc::set_handler(move || {
        println!("\n exiting black_jack... for now.");
        p.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
        
    let mut dealer = Player{ hand : Vec::new() , money : 10000};
    let mut player = Player{ hand : Vec::new() , money : 1000};
    
    
    let mut deck = Deck::new();
    Deck::shuffle(&mut deck);


    clearscreen::clear().expect("failed to clear screen");
    println!("Let's play black_jack");
    
    while playing.load(Ordering::SeqCst) {
        if !playing.load(Ordering::SeqCst) {
            break;
        }


    }
}   
