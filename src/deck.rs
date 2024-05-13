use crate::{card::Card, hand::Hand};

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn standard() -> Deck {
        let mut deck = Deck { cards: vec![] };

        while deck.cards.len() < 52 {
            for i in 0..4 {
                let suit = Card::determine_suit(i);

                for i in 2..15 {
                    match i {
                        11 => deck
                            .cards
                            .push(Card::new(suit.clone(), "Jack".to_string(), i)),
                        12 => deck
                            .cards
                            .push(Card::new(suit.clone(), "Queen".to_string(), i)),
                        13 => deck
                            .cards
                            .push(Card::new(suit.clone(), "King".to_string(), i)),
                        14 => deck
                            .cards
                            .push(Card::new(suit.clone(), "Ace".to_string(), i)),

                        _ => deck.cards.push(Card::new(suit.clone(), i.to_string(), i)),
                    };
                }
            }
        }

        deck
    }

    pub fn deal_to_hand(&mut self, hand: &mut Hand) {
        while hand.cards.len() < hand.size {
            hand.cards.push(self.cards.pop().unwrap());
        }
    }

    pub fn deal_n_cards(&mut self, n: usize) -> Vec<Card> {
        let mut draws: Vec<Card> = vec![];
        for _ in 0..n {
            if self.cards.len() == 0 {
                return draws
            } else {
                draws.push(self.cards.pop().unwrap());
            }
        }

        return draws;
    }
}
