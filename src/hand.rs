use std::collections::HashMap;

use crate::{
    card::Card,
    deck::Deck,
    suit::Suits,
    valid_hands::{self, ValidHands},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub size: usize,
}

impl Hand {
    pub fn default() -> Hand {
        Hand {
            cards: vec![],
            size: 5,
        }
    }

    pub fn of_size(hand_size: usize) -> Hand {
        Hand {
            cards: vec![],
            size: hand_size,
        }
    }

    pub fn add_to_hand(&mut self, mut cards: Vec<Card>) {
        while cards.len() > 0 {
            self.cards.push(cards.remove(0));
        }
    }

    pub fn contains_ace(&self) -> bool {
        let mut has_ace = false;
        for card in &self.cards {
            if card.value == 14 {
                has_ace = true;
                break;
            }
        }

        has_ace
    }

    pub fn held_suits(&self) -> HashMap<Suits, Vec<usize>> {
        let mut suit_map: HashMap<Suits, Vec<usize>> = HashMap::new();
        for c in &self.cards {
            suit_map.entry(c.suit).or_insert(Vec::new()).push(c.value);
        }
        suit_map
    }

    pub fn held_suits_as_hands(&self) -> HashMap<Suits, Hand> {
        let mut suit_map: HashMap<Suits, Hand> = HashMap::new();
        for c in &self.cards {
            suit_map
                .entry(c.suit)
                .or_insert(Hand::default())
                .cards
                .push(c.clone());
        }
        suit_map
    }

    pub fn get_run_length_hashmap(&self) -> HashMap<usize, usize> {
        // Provides unordered run length encoding if card frequency is the only thing we care about.
        // eg. pairs, x of a kind etc.
        let mut hm = HashMap::new();
        for card in self.cards.iter() {
            hm.entry(card.value).and_modify(|rl| *rl += 1).or_insert(1);
        }

        hm
    }

    pub fn get_run_length_tuples(&self, ace_high: bool) -> Vec<(usize, usize)> {
        // Provides run length encoding for the hand in card value sequential order.
        // important for checking for straights.

        let ace_hi = |card_val: usize| match (ace_high, card_val) {
            // ensures Ace value corresponds to high or low sorting.
            (true, 14) => 14,
            (false, 14) => 1,
            _ => card_val,
        };
        let mut result = vec![];

        let Some(&Card {
            value: mut largest_card_value,
            ..
        }) = self.cards.first()
        else {
            return result;
        };

        // In Ace-Low ordering the first card will be an Ace, but should be valued at 1.
        largest_card_value = ace_hi(largest_card_value);

        let mut run_length = 0;
        for card in self.cards.iter().skip(1) {
            run_length += 1;
            if card.value > largest_card_value {
                result.push((run_length, largest_card_value));
                run_length = 0;
                largest_card_value = ace_hi(card.value);
            }
        }
        if run_length == 0 {
            // the case where the final run is only the final card.
            result.push((1, largest_card_value))
        }
        result
    }

    pub fn contains(&self) -> Vec<ValidHands> {
        let mut valid_hands: Vec<ValidHands> = vec![];

        // Straight Flush here

        if let Some((four_oak, _)) = ValidHands::has_n_of_a_kind(self.clone(), 4) {
            match four_oak {
                ValidHands::FourOAK(_, _, _, _) => valid_hands.push(four_oak),
                _ => panic!(),
            }
        }

        // Full House Here

        if let Some((flush, _)) = ValidHands::detect_flush(self.clone()) {
            match flush {
                ValidHands::Flush(_, _, _, _, _) => valid_hands.push(flush),
                _ => panic!(),
            }
        }

        if let Some((straight, _)) = ValidHands::has_straight(self.clone()) {
            match straight {
                ValidHands::Straight(_, _, _, _, _) => valid_hands.push(straight),
                _ => panic!(),
            };
        }

        if let Some((three_oak, _)) = ValidHands::has_n_of_a_kind(self.clone(), 3) {
            match three_oak {
                ValidHands::ThreeOAK(_, _, _) => valid_hands.push(three_oak),
                _ => panic!(),
            }
        }

        // Two Pair Here

        if let Some((pair, _)) = ValidHands::has_n_of_a_kind(self.clone(), 2) {
            match pair {
                ValidHands::Pair(_, _) => valid_hands.push(pair),
                _ => panic!(),
            }
        }

        return valid_hands;
    }

    pub fn sort_by_rank_ace_high(&mut self) {
        self.cards
            .sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap());

        self.cards.reverse();
    }

    pub fn sort_by_rank_ace_low(&mut self) {
        self.cards
            .sort_by(|a, b| a.alt_value.partial_cmp(&b.alt_value).unwrap())
    }

    pub fn sort_by_suit(&mut self) {
        self.cards
            .sort_by(|a, b| b.suit.partial_cmp(&a.suit).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        card::Card,
        suit::{Ranks, Suits},
    };

    use super::Hand;

    #[test]
    pub fn suit_buckets() {
        let mut hand = Hand::of_size(8);
        let cards = vec![
            Card::new(Suits::Spades, Ranks::Ace, 14),
            Card::new(Suits::Spades, Ranks::Ten, 10),
            Card::new(Suits::Spades, Ranks::Nine, 9),
            Card::new(Suits::Hearts, Ranks::Ace, 14),
            Card::new(Suits::Hearts, Ranks::Ten, 10),
            Card::new(Suits::Hearts, Ranks::Nine, 9),
            Card::new(Suits::Clubs, Ranks::Ace, 14),
            Card::new(Suits::Diamonds, Ranks::Ace, 14),
        ];
        hand.cards = cards;
        let mut h = HashMap::new();
        h.insert(Suits::Spades, vec![14, 10, 9]);
        h.insert(Suits::Hearts, vec![14, 10, 9]);
        h.insert(Suits::Clubs, vec![14]);
        h.insert(Suits::Diamonds, vec![14]);
        let suit_map = hand.held_suits();
        assert_eq!(suit_map, h)
    }
}
