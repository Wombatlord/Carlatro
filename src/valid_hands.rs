use std::{collections::{hash_set, HashSet}, fmt};

use crate::{card::Card, hand::Hand};

#[derive(Debug, Clone, PartialEq)]
pub enum ValidHands {
    Pair(Card, Card),
    TwoPair(Card, Card, Card, Card),
    ThreeOAK(Card, Card, Card),
    Straight(Card, Card, Card, Card, Card),
    Flush(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    FourOAK(Card, Card, Card, Card),
    StraightFlush(Card, Card, Card, Card, Card),
}

impl fmt::Display for ValidHands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidHands::Pair(c1, c2) => {
                write!(f, "Pair:\n{}\n{}\n", c1, c2)
            }
            ValidHands::TwoPair(c1, c2, c3, c4) => {
                write!(f, "Two Pair:\n{}\n{}\n{}\n{}\n", c1, c2, c3, c4)
            }
            ValidHands::ThreeOAK(c1, c2, c3) => write!(f, "3OAK:\n{}\n{}\n{}\n", c1, c2, c3),
            ValidHands::Straight(c1, c2, c3, c4, c5) => {
                write!(f, "Straight:\n{}\n{}\n{}\n{}\n{}\n", c1, c2, c3, c4, c5)
            }
            ValidHands::Flush(c1, c2, c3, c4, c5) => {
                write!(f, "Flush:\n{}\n{}\n{}\n{}\n{}\n", c1, c2, c3, c4, c5)
            }
            ValidHands::FullHouse(c1, c2, c3, c4, c5) => {
                write!(f, "Full House:\n{}\n{}\n{}\n{}\n{}\n", c1, c2, c3, c4, c5)
            }
            ValidHands::FourOAK(c1, c2, c3, c4) => {
                write!(f, "4OAK:\n{}\n{}\n{}\n{}\n", c1, c2, c3, c4)
            }
            ValidHands::StraightFlush(c1, c2, c3, c4, c5) => {
                write!(
                    f,
                    "Straight Flush:\n{}\n{}\n{}\n{}\n{}\n",
                    c1, c2, c3, c4, c5
                )
            }
        }
    }
}

impl ValidHands {
    pub fn has_pair(hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut working_hand = hand.clone();
        let iter = working_hand.cards.as_slice().windows(2);

        for (idx, cards) in iter.enumerate() {
            if cards[0].value == cards[1].value {
                return Some((
                    ValidHands::Pair(
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                    ),
                    working_hand,
                ));
            }
        }

        None
    }

    pub fn has_two_pair(hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut paired_cards: Vec<Card> = vec![];
        if let Some((first_pair, leftover)) = Self::has_pair(hand.clone()) {
            match first_pair {
                ValidHands::Pair(first, second) => {
                    paired_cards.push(first);
                    paired_cards.push(second)
                }
                _ => panic!(),
            }

            if let Some((sp, final_leftover)) = Self::has_pair(leftover) {
                match sp {
                    ValidHands::Pair(first, second) => {
                        return Some((
                            ValidHands::TwoPair(
                                paired_cards.remove(0),
                                paired_cards.remove(0),
                                first,
                                second,
                            ),
                            final_leftover,
                        ));
                    }
                    _ => panic!(),
                };
            }
        }
        return None;
    }

    pub fn has_three_oak(hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut working_hand = hand.clone();

        let iter = working_hand.cards.as_slice().windows(3);

        for (idx, cards) in iter.enumerate() {
            if cards[0].value == cards[1].value && cards[0].value == cards[2].value {
                return Some((
                    ValidHands::ThreeOAK(
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                    ),
                    working_hand,
                ));
            }
        }

        None
    }

    pub fn has_straight(hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut working_hand = hand.clone();

        working_hand.sort_by_rank_ace_high();
        remove_duplicates(&mut working_hand); // Ensure (A, A, A, 2, 3, 3, 4, 5) is picked up as straight.
        
        // check for all straights, ace high ordering (A,K,Q,J,10 ... 2)
        let iter = working_hand.cards.as_slice().windows(5);
        for (idx, cards) in iter.clone().enumerate() {
            if cards[0].value == cards[1].value + 1
                && cards[1].value == cards[2].value + 1
                && cards[2].value == cards[3].value + 1
                && cards[3].value == cards[4].value + 1
            {
                return Some((
                    ValidHands::Straight(
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                    ),
                    working_hand,
                ));
            }
        }

        // check for A-2-3-4-5
        working_hand.sort_by_rank_ace_low();
        if working_hand.cards[0].rank == "Ace" {
            let iter = working_hand.cards.as_slice().windows(5);
            for (idx, cards) in iter.enumerate() {
                if cards[0].rank == "Ace"
                    && cards[0].alt_value == cards[1].value - 1
                    && cards[1].value == cards[2].value - 1
                    && cards[2].value == cards[3].value - 1
                    && cards[3].value == cards[4].value - 1
                {
                    return Some((
                        ValidHands::Straight(
                            working_hand.cards.remove(idx),
                            working_hand.cards.remove(idx),
                            working_hand.cards.remove(idx),
                            working_hand.cards.remove(idx),
                            working_hand.cards.remove(idx),
                        ),
                        working_hand,
                    ));
                }
            }
        }

        None
    }

    pub fn has_flush(hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut working_hand = hand.clone();
        working_hand.sort_by_suit();

        let iter = working_hand.cards.as_slice().windows(5);
        for (idx, cards) in iter.enumerate() {
            if cards[0].suit == cards[1].suit
                && cards[0].suit == cards[2].suit
                && cards[0].suit == cards[3].suit
                && cards[0].suit == cards[4].suit
            {
                let mut h = Hand {
                    cards: vec![
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                    ],
                    size: 0,
                };

                h.sort_by_rank_ace_high();
                return Some((
                    ValidHands::Flush(
                        h.cards[0].clone(),
                        h.cards[1].clone(),
                        h.cards[2].clone(),
                        h.cards[3].clone(),
                        h.cards[4].clone(),
                    ),
                    working_hand,
                ));
            }
        }

        None
    }

    pub fn has_full_house(hand: Hand) -> Option<(ValidHands, Hand)> {
        let working_hand = hand.clone();
        let three_oak_and_leftover = Self::has_three_oak(working_hand);
        if three_oak_and_leftover.is_none() {
            return None;
        }

        let (three_oak, left_over) = three_oak_and_leftover.unwrap();
        let pair_and_leftover = Self::has_pair(left_over);
        if pair_and_leftover.is_none() {
            return None;
        }
        let (pair, left_over) = pair_and_leftover.unwrap();
        match three_oak {
            ValidHands::ThreeOAK(a, b, c) => match pair {
                ValidHands::Pair(d, e) => {
                    return Some((ValidHands::FullHouse(a, b, c, d, e), left_over))
                }
                _ => panic!("Unexpected Hand: {pair}, expected: ValidHands::Pair"),
            },
            _ => panic!("Unexpected Hand: {three_oak}, expected: ValidHands::ThreeOAK"),
        }
    }

    pub fn has_four_oak(hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut working_hand = hand.clone();

        let iter = working_hand.cards.as_slice().windows(4);

        for (idx, cards) in iter.enumerate() {
            if cards[0].value == cards[1].value
                && cards[0].value == cards[2].value
                && cards[0].value == cards[3].value
            {
                return Some((
                    ValidHands::FourOAK(
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                        working_hand.cards.remove(idx),
                    ),
                    working_hand,
                ));
            }
        }

        None
    }

    pub fn has_straight_variant(hand: Hand) -> Option<(ValidHands, Hand)> {
        let working_hand = hand.clone();
        let maybe_straight = Self::has_straight(working_hand);
        if maybe_straight.is_none() {
            return None;
        }
        let (confirmed_straight, left_over) = maybe_straight.unwrap();

        // Here we check for a straight
        let straight_hand = match confirmed_straight {
            ValidHands::Straight(card_1, card_2, card_3, card_4, card_5) => {
                let hand = Hand {
                    cards: vec![card_1, card_2, card_3, card_4, card_5],
                    size: 5,
                };
                hand
            }
            _ => panic!("Unexpected Hand: {confirmed_straight}. Expected: ValidHands::Straight"),
        };

        // Here we check for a flush
        let maybe_flush = Self::has_flush(straight_hand.clone());
        if maybe_flush.is_none() {
            return Some((ValidHands::Straight(
                straight_hand.cards[0].clone(),
                straight_hand.cards[1].clone(),
                straight_hand.cards[2].clone(),
                straight_hand.cards[3].clone(),
                straight_hand.cards[4].clone(),
            ), left_over));
        }
        
        let (confirmed_flush, _) = maybe_flush.unwrap();
        match confirmed_flush {
            ValidHands::Flush(card_1, card_2, card_3, card_4, card_5) => {
                return Some((
                    ValidHands::StraightFlush(card_1, card_2, card_3, card_4, card_5),
                    left_over,
                ))
            }
            _ => panic!("Unexpected Hand: {confirmed_flush}. Expected: ValidHands::Straight"),
        }
    }
}

fn remove_duplicates(working_hand: &mut Hand) {
    // (2C, 3H, 3S, 4C, 5C, 6D, 10D, KD) : This hand is a problem due to pair inside the straight.
    let it = working_hand.clone();
    let itr = it.cards.windows(4);
    for (idx, c) in itr.enumerate() {
        if c[0].value == c[1].value {
            // println!("{} : {}", c[0], c[1]);
            working_hand.cards.remove(idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{card::Card, hand::Hand, valid_hands::ValidHands};

    // #[test]
    fn finds_pair() {
        let mut hand = Hand::of_size(8);
        let pair_card_a = Card::new("Hearts".into(), "2".into(), 2);
        let pair_card_b = Card::new("Spades".into(), "2".into(), 2);
        let cards = vec![
            pair_card_a.clone(),
            Card::new("Spades".into(), "3".into(), 3),
            Card::new("Spades".into(), "4".into(), 4),
            Card::new("Spades".into(), "5".into(), 5),
            Card::new("Spades".into(), "6".into(), 6),
            Card::new("Spades".into(), "7".into(), 7),
            Card::new("Spades".into(), "8".into(), 8),
            pair_card_b.clone(),
        ];

        hand.cards = cards;
        hand.sort_by_rank_ace_high();
        if let Some((vh, _)) = ValidHands::has_pair(hand.clone()) {
            println!("{vh}");
            assert_eq!(vh, ValidHands::Pair(pair_card_a, pair_card_b));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn finds_middle_straight_ace_high_ordering() {
        let mut hand = Hand::of_size(8);
        let straight_card_a = Card::new("Hearts".into(), "8".into(), 8);
        let straight_card_b = Card::new("Spades".into(), "7".into(), 7);
        let straight_card_c = Card::new("Clubs".into(), "6".into(), 6);
        let straight_card_d = Card::new("Diamonds".into(), "5".into(), 5);
        let straight_card_e = Card::new("Spades".into(), "4".into(), 4);
        hand.cards = vec![
            straight_card_a.clone(),
            Card::new("Clubs".into(), "Jack".into(), 11),
            straight_card_c.clone(),
            Card::new("Diamonds".into(), "Jack".into(), 11),
            Card::new("Clubs".into(), "King".into(), 13),
            straight_card_d.clone(),
            straight_card_e.clone(),
            straight_card_b.clone(),
        ];

        if let Some((vh, _)) = ValidHands::has_straight(hand.clone()) {
            println!("{vh}");
            assert_eq!(
                vh,
                ValidHands::Straight(
                    straight_card_a,
                    straight_card_b,
                    straight_card_c,
                    straight_card_d,
                    straight_card_e
                )
            );
        } else {
            assert!(false);
        }
    }

    #[test]
    fn finds_ace_high_straight() {
        let mut hand = Hand::of_size(8);
        let straight_card_a = Card::new("Hearts".into(), "Ace".into(), 14);
        let straight_card_b = Card::new("Spades".into(), "King".into(), 13);
        let straight_card_c = Card::new("Clubs".into(), "Queen".into(), 12);
        let straight_card_d = Card::new("Diamonds".into(), "Jack".into(), 11);
        let straight_card_e = Card::new("Spades".into(), "10".into(), 10);
        hand.cards = vec![
            straight_card_a.clone(),
            Card::new("Clubs".into(), "7".into(), 7),
            straight_card_c.clone(),
            Card::new("Clubs".into(), "4".into(), 4),
            Card::new("Clubs".into(), "3".into(), 3),
            straight_card_d.clone(),
            straight_card_e.clone(),
            straight_card_b.clone(),
        ];

        if let Some((vh, _)) = ValidHands::has_straight(hand.clone()) {
            println!("{vh}");
            assert_eq!(
                vh,
                ValidHands::Straight(
                    straight_card_a,
                    straight_card_b,
                    straight_card_c,
                    straight_card_d,
                    straight_card_e
                )
            );
        } else {
            assert!(
                false,
                "{}",
                format!("No Straight found. Hand: {:?}", hand.cards)
            );
        }
    }

    #[test]
    fn finds_ace_low_straight() {
        let mut hand = Hand::of_size(8);
        let straight_card_a = Card::new("Hearts".into(), "Ace".into(), 14);
        let straight_card_b = Card::new("Spades".into(), "2".into(), 13);
        let straight_card_c = Card::new("Clubs".into(), "3".into(), 12);
        let straight_card_d = Card::new("Diamonds".into(), "4".into(), 11);
        let straight_card_e = Card::new("Spades".into(), "5".into(), 10);
        hand.cards = vec![
            straight_card_a.clone(),
            Card::new("Clubs".into(), "7".into(), 7),
            straight_card_c.clone(),
            Card::new("Clubs".into(), "4".into(), 4),
            Card::new("Clubs".into(), "8".into(), 8),
            straight_card_d.clone(),
            straight_card_e.clone(),
            straight_card_b.clone(),
        ];

        if let Some((vh, _)) = ValidHands::has_straight(hand.clone()) {
            println!("{vh}");
            assert_eq!(
                vh,
                ValidHands::Straight(
                    straight_card_a,
                    straight_card_b,
                    straight_card_c,
                    straight_card_d,
                    straight_card_e
                )
            );
        } else {
            assert!(false);
        }
    }

    #[test]
    fn finds_flush() {
        let mut hand = Hand::of_size(8);
        let flush_card_a = Card::new("Hearts".into(), "Ace".into(), 14);
        let flush_card_b = Card::new("Hearts".into(), "Jack".into(), 11);
        let flush_card_c = Card::new("Hearts".into(), "8".into(), 8);
        let flush_card_d = Card::new("Hearts".into(), "6".into(), 6);
        let flush_card_e = Card::new("Hearts".into(), "3".into(), 3);
        let cards = vec![
            flush_card_a.clone(),
            Card::new("Clubs".into(), "Jack".into(), 11),
            flush_card_c.clone(),
            Card::new("Clubs".into(), "3".into(), 3),
            Card::new("Clubs".into(), "8".into(), 8),
            flush_card_d.clone(),
            flush_card_e.clone(),
            flush_card_b.clone(),
        ];

        hand.cards = cards;
        if let Some((vh, _)) = ValidHands::has_flush(hand.clone()) {
            println!("{vh}");
            assert_eq!(
                vh,
                ValidHands::Flush(
                    flush_card_a,
                    flush_card_b,
                    flush_card_c,
                    flush_card_d,
                    flush_card_e
                )
            );
        } else {
            assert!(false);
        }
    }
}
