use crate::{card::Card, hand::Hand, suit::Ranks};

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
                            .push(Card::new(suit.clone(), Ranks::Jack, i)),
                        12 => deck
                            .cards
                            .push(Card::new(suit.clone(), Ranks::Queen, i)),
                        13 => deck
                            .cards
                            .push(Card::new(suit.clone(), Ranks::King, i)),
                        14 => deck
                            .cards
                            .push(Card::new(suit.clone(), Ranks::Ace, i)),

                        _ => deck.cards.push(Card::new(suit.clone(), Ranks::get_by_value(i), i)),
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
