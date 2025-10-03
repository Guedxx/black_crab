use crate::deck::Card;

struct Player {
    pub hand: Vec<Card>,
    pub money: i32,
}

impl Player {
    pub fn new() -> Self {
        let hand = Vec::new();
        let money = 0;
        Player { hand, money }
    } 
}
