use std::{
    collections::{hash_set, HashSet},
    fmt,
};

use crate::{
    card::{self, Card},
    hand::Hand,
    suit::Ranks,
};

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
    pub fn has_n_of_a_kind(hand: Hand, n: usize) -> Option<(ValidHands, Hand)> {
        let mut detected_n_oak = None;
        let rle = hand.get_run_length_hashmap();
        println!("{rle:?}");
        for (_card_value, amount_in_hand) in rle {
            if amount_in_hand == n {
                match n {
                    2 => {
                        detected_n_oak =
                            Some((ValidHands::Pair(Card::blank(), Card::blank()), hand.clone()))
                    }
                    3 => {
                        detected_n_oak = Some((
                            ValidHands::ThreeOAK(Card::blank(), Card::blank(), Card::blank()),
                            hand.clone(),
                        ))
                    }
                    4 => {
                        detected_n_oak = Some((
                            ValidHands::FourOAK(
                                Card::blank(),
                                Card::blank(),
                                Card::blank(),
                                Card::blank(),
                            ),
                            hand.clone(),
                        ))
                    }
                    _ => continue,
                }
            }
        }

        detected_n_oak
    }

    pub fn has_two_pair(hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut detected_two_pair = None;
        let rle = hand.get_run_length_hashmap();
        let mut pair_a = false;
        let mut pair_b = false;

        for (_, v) in rle {
            if pair_a == false && v >= 2 {
                pair_a = true
            } else if pair_a == true && v >= 2 {
                pair_b = true
            }
        }

        match (pair_a, pair_b) {
            (true, true) => {
                detected_two_pair = Some((
                    ValidHands::TwoPair(Card::blank(), Card::blank(), Card::blank(), Card::blank()),
                    hand,
                ))
            }
            _ => (),
        }

        detected_two_pair
    }

    pub fn has_full_house(hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut detected_full_house = None;

        let rle = hand.get_run_length_hashmap();
        let mut found_pair = false;
        let mut found_trio = false;

        for (_, v) in rle {
            if v >= 3 {
                found_trio = true;
            } else if v >= 2 {
                found_pair = true;
            }
        }

        if found_pair && found_trio {
            detected_full_house = Some((
                ValidHands::FullHouse(
                    Card::blank(),
                    Card::blank(),
                    Card::blank(),
                    Card::blank(),
                    Card::blank(),
                ),
                hand,
            ))
        }

        detected_full_house
    }

    pub fn has_straight_flush(hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut detected_straight_flush = None;
        let suit_map = hand.held_suits_as_hands();
        for (_suit, suited_hand) in suit_map {
            if suited_hand.cards.len() >= 5 {
                let rle_straight = Self::has_straight(suited_hand.clone());

                if rle_straight.is_some() {
                    detected_straight_flush = Some((
                        ValidHands::StraightFlush(
                            suited_hand.cards[0],
                            suited_hand.cards[1],
                            suited_hand.cards[2],
                            suited_hand.cards[3],
                            suited_hand.cards[4],
                        ),
                        hand.clone(),
                    ));
                }
            }
        }

        detected_straight_flush
    }

    fn detect_straight(hand: Hand, ace_high: bool) -> Option<(ValidHands, Hand)> {
        let mut detected_straight = None;
        let rle_tuples = hand.get_run_length_tuples(ace_high);

        for candidate in rle_tuples.windows(5) {
            let mut pairs = candidate.windows(2);
            let mut straight = vec![candidate[0].1];
            while let Some(&[(_, current), (_, next)]) = pairs.next() {
                let pair_diff = next
                    .checked_sub(current)
                    .expect("RLE tuples did not have expected ascending order");
                if pair_diff > 1 {
                    break;
                } else {
                    straight.push(next)
                }
            }
            if straight.len() == 5 {
                detected_straight = Some((
                    ValidHands::Straight(
                        Card::blank(),
                        Card::blank(),
                        Card::blank(),
                        Card::blank(),
                        Card::blank(),
                    ),
                    hand.clone(),
                ));
            }
        }

        detected_straight
    }

    pub fn has_straight(mut hand: Hand) -> Option<(ValidHands, Hand)> {
        hand.sort_by_rank_ace_high();
        let mut detected = Self::detect_straight(hand.clone(), true);

        if detected.is_none() && hand.contains_ace() {
            hand.sort_by_rank_ace_low();
            detected = Self::detect_straight(hand, false);
        }

        detected
    }

    pub fn detect_flush(hand: Hand) -> Option<(ValidHands, Hand)> {
        let suit_map = hand.held_suits();

        for (suit, v) in suit_map.clone() {
            if v.len() >= 5 {
                return Some((
                    ValidHands::Flush(
                        Card::from_card_value(v[0], Some(suit)),
                        Card::from_card_value(v[1], Some(suit)),
                        Card::from_card_value(v[2], Some(suit)),
                        Card::from_card_value(v[3], Some(suit)),
                        Card::from_card_value(v[4], Some(suit)),
                    ),
                    hand,
                ));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{card::Card, hand::Hand, suit::Suits, valid_hands::ValidHands};
    #[test]
    fn finds_n_of_a_kind() {
        let hands = vec![
            // this case fails if run length encoding fails to catch matching highest card values
            Hand {
                cards: vec![
                    Card::from_card_value(14, Some(Suits::Hearts)),
                    Card::from_card_value(14, Some(Suits::Spades)),
                    Card::from_card_value(14, Some(Suits::Clubs)),
                    Card::from_card_value(10, Some(Suits::Spades)),
                    Card::from_card_value(12, Some(Suits::Spades)),
                    Card::from_card_value(5, Some(Suits::Spades)),
                    Card::from_card_value(6, Some(Suits::Spades)),
                    Card::from_card_value(11, Some(Suits::Hearts)),
                ],
                size: 8,
            },
            // this case fails if run length encoding fails to catch pair
            Hand {
                cards: vec![
                    Card::from_card_value(7, Some(Suits::Hearts)),
                    Card::from_card_value(7, Some(Suits::Spades)),
                    Card::from_card_value(3, Some(Suits::Spades)),
                    Card::from_card_value(4, Some(Suits::Spades)),
                    Card::from_card_value(5, Some(Suits::Spades)),
                    Card::from_card_value(6, Some(Suits::Spades)),
                    Card::from_card_value(12, Some(Suits::Clubs)),
                    Card::from_card_value(11, Some(Suits::Hearts)),
                ],
                size: 8,
            },
            // this case fails if run length encoding fails to catch 4oak
            Hand {
                cards: vec![
                    Card::from_card_value(7, Some(Suits::Hearts)),
                    Card::from_card_value(7, Some(Suits::Spades)),
                    Card::from_card_value(7, Some(Suits::Spades)),
                    Card::from_card_value(7, Some(Suits::Spades)),
                    Card::from_card_value(5, Some(Suits::Spades)),
                    Card::from_card_value(6, Some(Suits::Spades)),
                    Card::from_card_value(12, Some(Suits::Clubs)),
                    Card::from_card_value(11, Some(Suits::Hearts)),
                ],
                size: 8,
            },
        ];

        for (idx, mut hand) in hands.into_iter().enumerate() {
            hand.sort_by_rank_ace_high();
            match idx {
                0 => {
                    assert_eq!(
                        ValidHands::has_n_of_a_kind(hand.clone(), 3),
                        Some((
                            ValidHands::ThreeOAK(Card::blank(), Card::blank(), Card::blank()),
                            hand
                        )),
                        "Failed on case {}",
                        idx
                    )
                }
                1 => {
                    assert_eq!(
                        ValidHands::has_n_of_a_kind(hand.clone(), 2),
                        Some((ValidHands::Pair(Card::blank(), Card::blank()), hand)),
                        "Failed on case {}",
                        idx
                    )
                }
                2 => {
                    assert_eq!(
                        ValidHands::has_n_of_a_kind(hand.clone(), 4),
                        Some((
                            ValidHands::FourOAK(
                                Card::blank(),
                                Card::blank(),
                                Card::blank(),
                                Card::blank()
                            ),
                            hand
                        )),
                        "Failed on case {}",
                        idx
                    )
                }
                _ => panic!("Index out of range. Missing test cases."),
            }
        }
    }

    #[test]
    fn finds_two_pair() {
        let hands = vec![
            // this case fails if run length encoding fails to catch matching highest card values
            Hand {
                cards: vec![
                    Card::from_card_value(14, Some(Suits::Hearts)),
                    Card::from_card_value(14, Some(Suits::Spades)),
                    Card::from_card_value(10, Some(Suits::Clubs)),
                    Card::from_card_value(10, Some(Suits::Spades)),
                    Card::from_card_value(12, Some(Suits::Spades)),
                    Card::from_card_value(5, Some(Suits::Spades)),
                    Card::from_card_value(6, Some(Suits::Spades)),
                    Card::from_card_value(11, Some(Suits::Hearts)),
                ],
                size: 8,
            },
        ];

        for (idx, hand) in hands.into_iter().enumerate() {
            assert_eq!(
                ValidHands::has_two_pair(hand.clone()),
                Some((
                    ValidHands::TwoPair(Card::blank(), Card::blank(), Card::blank(), Card::blank(),),
                    hand
                )),
                "Failed on case {}",
                idx
            )
        }
    }

    #[test]
    fn finds_flush() {
        let hand = Hand {
            cards: vec![
                Card::from_card_value(14, Some(Suits::Spades)),
                Card::from_card_value(3, Some(Suits::Spades)),
                Card::from_card_value(4, Some(Suits::Spades)),
                Card::from_card_value(5, Some(Suits::Spades)),
                Card::from_card_value(6, Some(Suits::Spades)),
                Card::from_card_value(10, Some(Suits::Hearts)),
                Card::from_card_value(11, Some(Suits::Hearts)),
                Card::from_card_value(12, Some(Suits::Clubs)),
            ],
            size: 8,
        };

        assert_eq!(
            ValidHands::detect_flush(hand.clone()),
            Some((
                ValidHands::Flush(
                    hand.cards[0].clone(),
                    hand.cards[1].clone(),
                    hand.cards[2].clone(),
                    hand.cards[3].clone(),
                    hand.cards[4].clone(),
                ),
                hand
            ))
        )
    }

    #[test]
    fn finds_straight() {
        let hands = vec![
            // this case fails if check is sensitive to card value duplication
            Hand {
                cards: vec![
                    Card::from_card_value(2, None),
                    Card::from_card_value(3, None),
                    Card::from_card_value(3, None),
                    Card::from_card_value(4, None),
                    Card::from_card_value(4, None),
                    Card::from_card_value(4, None),
                    Card::from_card_value(5, None),
                    Card::from_card_value(6, None),
                ],
                size: 8,
            },
            // this case fails if ace-low is not detected as start of straight
            Hand {
                cards: vec![
                    Card::from_card_value(14, None),
                    Card::from_card_value(2, None),
                    Card::from_card_value(3, None),
                    Card::from_card_value(4, None),
                    Card::from_card_value(5, None),
                    Card::from_card_value(7, None),
                    Card::from_card_value(8, None),
                    Card::from_card_value(9, None),
                ],
                size: 8,
            },
            // this case fails if ace-high is not detected as end of straight
            Hand {
                cards: vec![
                    Card::from_card_value(4, None),
                    Card::from_card_value(5, None),
                    Card::from_card_value(6, None),
                    Card::from_card_value(10, None),
                    Card::from_card_value(11, None),
                    Card::from_card_value(12, None),
                    Card::from_card_value(13, None),
                    Card::from_card_value(14, None),
                ],
                size: 8,
            },
        ];
        for (idx, hand) in hands.into_iter().enumerate() {
            assert_eq!(
                ValidHands::has_straight(hand.clone()),
                Some((
                    ValidHands::Straight(
                        Card::blank(),
                        Card::blank(),
                        Card::blank(),
                        Card::blank(),
                        Card::blank(),
                    ),
                    hand,
                )),
                "Failed on case {}",
                idx
            );
        }
    }

    #[test]
    fn finds_full_house() {
        let hands = vec![
            // this case fails if check is sensitive to card value duplication
            Hand {
                cards: vec![
                    Card::from_card_value(4, Some(Suits::Hearts)),
                    Card::from_card_value(4, Some(Suits::Hearts)),
                    Card::from_card_value(7, Some(Suits::Hearts)),
                    Card::from_card_value(8, Some(Suits::Spades)),
                    Card::from_card_value(9, Some(Suits::Spades)),
                    Card::from_card_value(14, Some(Suits::Spades)),
                    Card::from_card_value(14, Some(Suits::Spades)),
                    Card::from_card_value(14, Some(Suits::Spades)),
                ],
                size: 8,
            },
        ];
        for (idx, hand) in hands.into_iter().enumerate() {
            assert_eq!(
                ValidHands::has_full_house(hand.clone()),
                Some((
                    ValidHands::FullHouse(
                        Card::blank(),
                        Card::blank(),
                        Card::blank(),
                        Card::blank(),
                        Card::blank(),
                    ),
                    hand,
                )),
                "Failed on case {}",
                idx
            );
        }
    }

    #[test]
    fn finds_straight_flush() {
        let hands = vec![
            // this case fails if check is sensitive to card value duplication
            Hand {
                cards: vec![
                    Card::from_card_value(2, Some(Suits::Spades)),
                    Card::from_card_value(3, Some(Suits::Spades)),
                    Card::from_card_value(4, Some(Suits::Spades)),
                    Card::from_card_value(5, Some(Suits::Spades)),
                    Card::from_card_value(6, Some(Suits::Spades)),
                    Card::from_card_value(3, Some(Suits::Hearts)),
                    Card::from_card_value(4, Some(Suits::Hearts)),
                    Card::from_card_value(4, Some(Suits::Hearts)),
                ],
                size: 8,
            },
            // this case fails if ace-low is not detected as start of straight
            Hand {
                cards: vec![
                    Card::from_card_value(2, Some(Suits::Spades)),
                    Card::from_card_value(3, Some(Suits::Spades)),
                    Card::from_card_value(4, Some(Suits::Spades)),
                    Card::from_card_value(5, Some(Suits::Spades)),
                    Card::from_card_value(14, Some(Suits::Spades)),
                    Card::from_card_value(7, Some(Suits::Hearts)),
                    Card::from_card_value(8, Some(Suits::Hearts)),
                    Card::from_card_value(9, Some(Suits::Hearts)),
                ],
                size: 8,
            },
            // this case fails if ace-high is not detected as end of straight
            Hand {
                cards: vec![
                    Card::from_card_value(10, Some(Suits::Spades)),
                    Card::from_card_value(11, Some(Suits::Spades)),
                    Card::from_card_value(12, Some(Suits::Spades)),
                    Card::from_card_value(13, Some(Suits::Spades)),
                    Card::from_card_value(14, Some(Suits::Spades)),
                    Card::from_card_value(4, Some(Suits::Hearts)),
                    Card::from_card_value(5, Some(Suits::Hearts)),
                    Card::from_card_value(6, Some(Suits::Hearts)),
                ],
                size: 8,
            },
        ];
        for (idx, hand) in hands.into_iter().enumerate() {
            assert_eq!(
                ValidHands::has_straight_flush(hand.clone()),
                Some((
                    ValidHands::StraightFlush(
                        hand.cards[0],
                        hand.cards[1],
                        hand.cards[2],
                        hand.cards[3],
                        hand.cards[4],
                    ),
                    hand,
                )),
                "Failed on case {}",
                idx
            );
        }
    }
}
