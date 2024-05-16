mod card;
mod deck;
mod hand;
mod valid_hands;
mod suit;

use deck::Deck;
use hand::Hand;
use rand::seq::SliceRandom;
use valid_hands::ValidHands;

fn main() {
    let samples = 100_000usize;
    let mut straight_flush = 0usize;
    let mut four_oak = 0usize;
    let mut full_house = 0usize;
    let mut flush = 0usize;
    let mut straight = 0usize;
    let mut three_oak = 0usize;
    let mut two_pair = 0usize;
    let mut pair = 0usize;

    let mut rng = rand::thread_rng();
    for _ in 0..samples {
        let mut deck = Deck::standard();
        deck.cards.shuffle(&mut rng);

        let mut hand = Hand::of_size(8);

        let first_draw = deck.deal_n_cards(8);
        hand.add_to_hand(first_draw);
        hand.sort_by_rank_ace_high();

        // let valid_hands = hand.contains();

        // for valid_hand in valid_hands {
        //     match valid_hand {
        //         ValidHands::StraightFlush(_, _, _, _, _) => straight_flush += 1,
        //         ValidHands::FourOAK(_, _, _, _) => four_oak += 1,
        //         ValidHands::FullHouse(_, _, _, _, _) => full_house += 1,
        //         ValidHands::Flush(_, _, _, _, _) => flush += 1,
        //         ValidHands::Straight(_, _, _, _, _) => straight += 1,
        //         ValidHands::ThreeOAK(_, _, _) => three_oak += 1,
        //         ValidHands::TwoPair(_, _, _, _) => two_pair += 1,
        //         ValidHands::Pair(_, _) => pair += 1,
        //     }
        // }

        let valid_hand = ValidHands::has_straight(hand.clone());
        if valid_hand.is_some() {
            let (v, _) = valid_hand.unwrap();
            match v {
                ValidHands::Straight(_, _, _, _, _) => straight += 1,
                // ValidHands::StraightFlush(_, _, _, _, _) => straight_flush += 1,
                _ => panic!()
            }
            // println!("\n---HIT---\n");
            // println!("{}", valid_hand.unwrap().0);
        }
    }

    let p = (straight as f32 / samples as f32) * 100.;
    println!("Found Straight in {p}% of first 8 card draw.");
    // let p = (straight_flush as f32 / samples as f32) * 100.;
    // println!("Found Straight Flush in {p}% of first 8 card draw.");

    // output_stats(
    //     samples,
    //     straight_flush,
    //     full_house,
    //     four_oak,
    //     flush,
    //     straight,
    //     three_oak,
    //     two_pair,
    //     pair,
    // );
}

fn output_stats(
    samples: usize,
    straight_flush: usize,
    full_house: usize,
    four_oak: usize,
    flush: usize,
    straight: usize,
    three_oak: usize,
    two_pair: usize,
    pair: usize,
) {
    println!("Samples: {samples}\n");
    let samples = samples as f32;
    let p = (straight_flush as f32 / samples) * 100.;
    println!("Found Straight Flush in {p}% of first 8 card draw.");

    let p = (full_house as f32 / samples) * 100.;
    println!("Found Full House in {p}% of first 8 card draw.");

    let p = (four_oak as f32 / samples) * 100.;
    println!("Found Four OAK in {p}% of first 8 card draw.");

    let p = (flush as f32 / samples) * 100.;
    println!("Found Flush in {p}% of first 8 card draw.");

    let p = (straight as f32 / samples) * 100.;
    println!("Found Straight in {p}% of first 8 card draw.");

    let p = (three_oak as f32 / samples) * 100.;
    println!("Found Three OAK in {p}% of first 8 card draw.");

    let p = (two_pair as f32 / samples) * 100.;
    println!("Found Two Pair in {p}% of first 8 card draw.");

    let p = (pair as f32 / samples) * 100.;
    println!("Found Pair in {p}% of first 8 card draw.");
}
