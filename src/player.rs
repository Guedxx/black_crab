use crate::deck::Card;

pub struct Player {
    pub hand: Vec<Card>,
    pub money: i32,
}

impl Player {
    pub fn create(money: i32) -> Player {
        let player = Player {
            hand: Vec::new(),
            money: money,
        };
        player
    }

    pub fn add_hand(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn print_cards(&self) {
        let mut lines = vec![String::new(); 7];

        for card in &self.hand {
            let rank = format!("{}", card.rank);
            let suit = format!("{}", card.suit);

            // Build each line of the card
            lines[0].push_str("┌─────────┐ ");
            lines[1].push_str(&format!("│{:<2}       │ ", rank));
            lines[2].push_str("│         │ ");
            lines[3].push_str(&format!("│    {}    │ ", suit));
            lines[4].push_str("│         │ ");
            lines[5].push_str(&format!("│       {:>2}│ ", rank));
            lines[6].push_str("└─────────┘ ");
        }

        // Print all lines
        for line in lines {
            println!("{}", line);
        }
    }

    pub fn sum_value_cards(&self) -> i32 {
        let mut sum = 0;
        let mut has_ace = false;

        for card in &self.hand {
            let value = card.rank.rank_int();
            sum += value;
            if value == 11 {
                has_ace = true;
            }
        }

        if sum > 21 && has_ace {
            sum -= 9;
        }
        sum
    }

    pub fn bet(&mut self, num: i32) {
        self.money -= num;
    }

    pub fn empty_hand(&mut self) {
        self.hand = Vec::new();
    }
}
