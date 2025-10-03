mod deck;
mod player;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;



fn main() {
    clearscreen::clear().expect("failed to clear screen");

    println!("Let's play black_jack");
    
    let playing = Arc::new(AtomicBool::new(true));
    let p = playing.clone();

    ctrlc::set_handler(move || {
        println!("\n exiting black_jack... for now.");
        p.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");


    while playing.load(Ordering::SeqCst) {
        if !playing.load(Ordering::SeqCst) {
            break;
        }   
    }
}   
