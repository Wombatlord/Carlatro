use std::fmt::Display;

use crate::suit::{Ranks, Suits};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Card {
    pub suit: Suits,
    pub rank: Ranks,
    pub value: usize,
    // alt_value for representing Ace-Low ordering.
    pub alt_value: usize,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.suit, self.rank.to_str())
    }
}

impl Card {
    pub fn blank() -> Card {
        Card {
            suit: Suits::Spades,
            rank: Ranks::Blank,
            value: Ranks::Blank.get_value(false),
            alt_value: 0,
        }
    }

    pub fn new(suit: Suits, rank: Ranks, mut value: usize) -> Card {
        let mut alt_value = value;

        if rank == Ranks::Ace {
            alt_value = 1;
            value = 14;
        }

        Card {
            suit,
            rank,
            value,
            alt_value,
        }
    }

    pub fn from_card_value(card_value: usize, mut suit: Option<Suits>) -> Self {
        if suit.is_none() {
            suit = Some(Suits::Spades);
        }
        let rank = match card_value {
            1 | 14 => Ranks::Ace,
            13 => Ranks::King,
            11 => Ranks::Jack,
            12 => Ranks::Queen,
            _ => Ranks::get_by_value(card_value),
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
