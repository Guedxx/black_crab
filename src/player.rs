use crate::card::Card;

struct Player {
    pub hand: Vec<Card>,
    pub money: i32,
}

impl Player {
    pub fn new() -> Self {
        let mut hand = Vec::new();
        let mut money = 0;
        Player { hand, money }
    } 
}
