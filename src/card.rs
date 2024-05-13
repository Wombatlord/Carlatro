use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Card {
    pub suit: String,
    pub rank: String,
    pub value: usize,
    // alt_value for representing Ace-Low ordering.
    pub alt_value: usize,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.suit, self.rank)
    }
}

impl Card {
    pub fn blank() -> Card {
        Card {suit: "".to_string(), rank: "".to_string(), value: 0, alt_value: 0}
    }

    pub fn new(suit: String, rank: String, value: usize) -> Card {
        let mut alt_value = value;
        
        if rank == "Ace" {
            alt_value = 1;
        }

        Card { suit, rank, value, alt_value }
    }

    pub fn determine_suit(i: i32) -> String {
        let suit: String;
        match i % 4 {
            0 => suit = "Hearts".to_string(),
            1 => suit = "Spades".to_string(),
            2 => suit = "Clubs".to_string(),
            3 => suit = "Diamonds".to_string(),
            _ => panic!(),
        }
        suit
    }
}