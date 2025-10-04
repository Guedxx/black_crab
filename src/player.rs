use crate::deck::Card;

pub struct Player {
    pub hand: Vec<Card>,
    pub money: i32,
}

impl Player {
    pub fn print_cards(&self) {
        // Create 7 lines for each card (card height)
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
} 

