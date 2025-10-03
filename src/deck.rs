use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

#[derive(Debug, Copy, Clone)]
pub enum Rank {
     Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King, Ace,
}

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub struct Deck {
    cards: Vec<Card>,

}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();

        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        let ranks = [
            Rank::Two, Rank::Three, Rank::Four, Rank::Five, 
            Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, 
            Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
        
        for suit in suits {
            for rank in ranks {
                cards.push(Card { rank, suit });
            }
        }
        
        Deck { cards }
    }
    
    pub fn shuffle(&mut self) {
       self.cards.shuffle(&mut thread_rng());
    }

}
