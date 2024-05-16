use std::{
    collections::{hash_set, HashSet},
    fmt,
};

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

    fn detect_three_oak(mut hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut detected_three_oak = None;
        let rle = hand.get_run_length_hashmap();
        for (_card_value, amount_in_hand) in rle {
            if amount_in_hand == 3 {
                detected_three_oak = Some((
                    ValidHands::ThreeOAK(Card::blank(), Card::blank(), Card::blank()),
                    hand.clone(),
                ));
            }
        }

        detected_three_oak
    }

    fn detect_rle_straight(hand: Hand, ace_high: bool) -> Option<(ValidHands, Hand)> {
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

    pub fn has_rle_straights(mut hand: Hand) -> Option<(ValidHands, Hand)> {
        hand.sort_by_rank_ace_high();
        let mut detected = Self::detect_rle_straight(hand.clone(), true);

        if detected.is_none() && hand.contains_ace() {
            hand.sort_by_rank_ace_low();
            detected = Self::detect_rle_straight(hand, false);
        }

        detected
    }

    pub fn has_straight(hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut working_hand = hand.clone();

        working_hand.sort_by_rank_ace_high();

        // Ensure (A, A, A, 2, 3, 3, 4, 5) is picked up as straight.
        let (mut stripped_hand, dupes) = remove_duplicates(&working_hand);

        // check for all straights, ace high ordering (A,K,Q,J,10 ... 2)
        let iter = stripped_hand.cards.as_slice().windows(5);
        for (idx, cards) in iter.clone().enumerate() {
            if cards[0].value == cards[1].value - 1
                && cards[1].value == cards[2].value - 1
                && cards[2].value == cards[3].value - 1
                && cards[3].value == cards[4].value - 1
            {
                return Some((
                    ValidHands::Straight(
                        stripped_hand.cards.remove(idx),
                        stripped_hand.cards.remove(idx),
                        stripped_hand.cards.remove(idx),
                        stripped_hand.cards.remove(idx),
                        stripped_hand.cards.remove(idx),
                    ),
                    dupes,
                ));
            }
        }

        // check for A-2-3-4-5
        stripped_hand.sort_by_rank_ace_low();
        if stripped_hand.cards[0].rank == "Ace" {
            let iter = stripped_hand.cards.as_slice().windows(5);
            for (idx, cards) in iter.enumerate() {
                if cards[0].rank == "Ace"
                    && cards[0].alt_value == cards[1].value - 1
                    && cards[1].value == cards[2].value - 1
                    && cards[2].value == cards[3].value - 1
                    && cards[3].value == cards[4].value - 1
                {
                    return Some((
                        ValidHands::Straight(
                            stripped_hand.cards.remove(idx),
                            stripped_hand.cards.remove(idx),
                            stripped_hand.cards.remove(idx),
                            stripped_hand.cards.remove(idx),
                            stripped_hand.cards.remove(idx),
                        ),
                        stripped_hand,
                    ));
                }
            }
        }

        None
    }

    pub fn has_flush(hand: Hand) -> Option<(ValidHands, Hand)> {
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

        //----------------------------
        // Here we check for a flush
        let maybe_flush = Self::has_flush(straight_hand.clone());
        if maybe_flush.is_none() {
            return Some((
                ValidHands::Straight(
                    straight_hand.cards[0].clone(),
                    straight_hand.cards[1].clone(),
                    straight_hand.cards[2].clone(),
                    straight_hand.cards[3].clone(),
                    straight_hand.cards[4].clone(),
                ),
                left_over,
            ));
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

fn remove_duplicates(working_hand: &Hand) -> (Hand, Hand) {
    // (2C, 3H, 3S, 4C, 5C, 6D, 10D, KD) : This hand is a problem due to pair inside the straight.
    let mut stripped = Hand::of_size(8);
    let mut dupes = Hand::of_size(8);
    let mut prev = Card::blank();
    for c in working_hand.clone().cards {
        if c.value != prev.value {
            stripped.cards.push(c.clone());
        } else {
            dupes.cards.push(c.clone())
        }
        prev = c;
    }

    (stripped, dupes)
}

#[cfg(test)]
mod tests {
    use crate::{card::Card, hand::Hand, suit::Suits, valid_hands::ValidHands};
    #[test]
    fn finds_three_oak() {
        let hands = vec![
            // this case fails if run length encoding fails to catch matching highest card values
            Hand {
                cards: vec![
                    Card::from_card_value(14, Some(Suits::Hearts)),
                    Card::from_card_value(14, Some(Suits::Spades)),
                    Card::from_card_value(14, Some(Suits::Clubs)),
                    Card::from_card_value(3, Some(Suits::Spades)),
                    Card::from_card_value(4, Some(Suits::Spades)),
                    Card::from_card_value(5, Some(Suits::Spades)),
                    Card::from_card_value(6, Some(Suits::Spades)),
                    Card::from_card_value(11, Some(Suits::Hearts)),
                ],
                size: 8,
            },
            // this case fails if run length encoding fails to catch matching highest card values
            Hand {
                cards: vec![
                    Card::from_card_value(7, Some(Suits::Hearts)),
                    Card::from_card_value(7, Some(Suits::Spades)),
                    Card::from_card_value(7, Some(Suits::Clubs)),
                    Card::from_card_value(3, Some(Suits::Spades)),
                    Card::from_card_value(4, Some(Suits::Spades)),
                    Card::from_card_value(5, Some(Suits::Spades)),
                    Card::from_card_value(6, Some(Suits::Spades)),
                    Card::from_card_value(11, Some(Suits::Hearts)),
                ],
                size: 8,
            },
        ];

        for mut hand in hands {
            hand.sort_by_rank_ace_high();

            assert_eq!(
                ValidHands::detect_three_oak(hand.clone()),
                Some((
                    ValidHands::ThreeOAK(Card::blank(), Card::blank(), Card::blank(),),
                    hand
                ))
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
            ValidHands::has_flush(hand.clone()),
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
    fn finds_rle_straight() {
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
                ValidHands::has_rle_straights(hand.clone()),
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
}
