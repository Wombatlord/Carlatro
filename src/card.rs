use std::fmt::Display;

use crate::suit::Suits;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Card {
    pub suit: Suits,
    pub rank: String,
    pub value: usize,
    // alt_value for representing Ace-Low ordering.
    pub alt_value: usize,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.suit, self.rank)
    }
}

impl Card {
    pub fn blank() -> Card {
        Card {suit: Suits::Spades, rank: "".to_string(), value: 0, alt_value: 0}
    }

    pub fn new(suit: Suits, rank: String, mut value: usize) -> Card {
        let mut alt_value = value;
        
        if rank == "Ace" {
            alt_value = 1;
            value = 14;
        }

        Card { suit, rank, value, alt_value }
    }

    pub fn from_card_value(card_value: usize, mut suit: Option<Suits>) -> Self {
        if suit.is_none() {
            suit = Some(Suits::Spades);
        }
        let rank = match card_value {
            1 | 14 => "Ace".to_string(),
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            _ => format!("{card_value}")
        };
    
        Card::new(suit.unwrap(), rank, card_value)
    }

    pub fn determine_suit(i: i32) -> Suits {
        let suit: Suits;
        match i % 4 {
            0 => suit = Suits::Spades,
            1 => suit = Suits::Hearts,
            2 => suit = Suits::Clubs,
            3 => suit = Suits::Diamonds,
            _ => panic!(),
        }
        suit
    }
}