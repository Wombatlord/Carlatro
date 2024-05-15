use crate::{card::Card, deck::Deck, valid_hands::{self, ValidHands}};

#[derive(Clone)]
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


    pub fn contains(&self) -> Vec<ValidHands>{
        let mut valid_hands: Vec<ValidHands> = vec![];
        if let Some((straight_flush, _)) = ValidHands::has_straight_variant(self.clone()) {
            match straight_flush {
                ValidHands::StraightFlush(_, _, _, _, _) => valid_hands.push(straight_flush),
                _ => panic!(),
            };
        } 
        
        if let Some((four_oak, _)) = ValidHands::has_four_oak(self.clone()) {
            match four_oak {
                ValidHands::FourOAK(_, _, _, _) => valid_hands.push(four_oak),
                _ => panic!()
            }
        }
        
        if let Some((full_house, _)) = ValidHands::has_full_house(self.clone()) {
            match full_house {
                ValidHands::FullHouse(_, _, _, _, _) => valid_hands.push(full_house),
                _ => panic!()
            }
        }

        if let Some((flush, _)) = ValidHands::has_flush(self.clone()) {
            match flush {
                ValidHands::Flush(_, _, _, _, _) => valid_hands.push(flush),
                _ => panic!()
            }
        }

        if let Some((three_oak, _)) = ValidHands::has_three_oak(self.clone()) {
            match three_oak {
                ValidHands::ThreeOAK(_, _, _) => valid_hands.push(three_oak),
                _ => panic!()
            }
        }

        if let Some((two_pair, _)) = ValidHands::has_two_pair(self.clone()) {
            match two_pair {
                ValidHands::TwoPair(_, _, _, _) => valid_hands.push(two_pair),
                _ => panic!()
            }
        }

        if let Some((pair, _)) = ValidHands::has_pair(self.clone()) {
            match pair {
                ValidHands::Pair(_, _) => valid_hands.push(pair),
                _ => panic!()
            }
        }

        return valid_hands;
    }

    pub fn sort_by_rank_ace_high(&mut self) {
        self.cards
            .sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap());
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
