use std::fmt;

use crate::{card::Card, hand::Hand};

#[derive(Debug, Clone)]
pub enum ValidHands {
    HighCard(Card),
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
            ValidHands::HighCard(card) => {
                write!(f, "High Card:\n{}\n", card)
            }
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
    // pub fn has_high_card(hand: Hand) -> Option<(ValidHands, Hand)> {
    //     let mut working_hand = hand.clone();
    //     let high_card = working_hand.cards.remove(0);
    //     Some((ValidHands::HighCard(high_card), working_hand))
    // }

    pub fn has_pair(hand: Hand) -> Option<(ValidHands, Hand)> {
        let mut working_hand = hand.clone();

        let mut prev_card = Card::blank();

        for i in 0..hand.cards.len() {
            if hand.cards[i].value == prev_card.value {
                return Some((
                    ValidHands::Pair(
                        working_hand.cards.swap_remove(i),
                        working_hand.cards.swap_remove(i - 1),
                    ),
                    working_hand.clone(),
                ));
            }
            prev_card = hand.cards[i].clone();
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
        let iter = working_hand.cards.as_slice().windows(5);
        // check for all 2-3-4-5-6 + straights
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

    pub fn has_straight_flush(hand: Hand) -> Option<(ValidHands, Hand)> {
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
        let maybe_flush = Self::has_flush(straight_hand);
        if maybe_flush.is_none() {
            return None;
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
