#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Suits {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
    Blank,
}

impl Suits {
    pub fn to_str(&self) -> &str {
        match self {
            Suits::Spades => "Spades",
            Suits::Hearts => "Hearts",
            Suits::Clubs => "Clubs",
            Suits::Diamonds => "Diamonds",
            Suits::Blank => "No Suit Set",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Ranks {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Blank,
}

impl Ranks {
    pub fn get_value(&self, is_low_ace: bool) -> usize {
        match self {
            Ranks::Ace => {
                if is_low_ace {
                    1
                } else {
                    14
                }
            }
            Ranks::King => 13,
            Ranks::Queen => 12,
            Ranks::Jack => 11,
            Ranks::Ten => 10,
            Ranks::Nine => 9,
            Ranks::Eight => 8,
            Ranks::Seven => 7,
            Ranks::Six => 6,
            Ranks::Five => 5,
            Ranks::Four => 4,
            Ranks::Three => 3,
            Ranks::Two => 2,
            Ranks::Blank => 0,
        }
    }

    pub fn get_by_value(value: usize) -> Self {
        match value {
            1 | 14 => Ranks::Ace,
            13 => Ranks::King,
            12 => Ranks::Queen,
            11 => Ranks::Jack,
            10 => Ranks::Ten,
            9 => Ranks::Nine,
            8 => Ranks::Eight,
            7 => Ranks::Seven,
            6 => Ranks::Six,
            5 => Ranks::Five,
            4 => Ranks::Four,
            3 => Ranks::Three,
            2 => Ranks::Two,
            0 => Ranks::Blank,
            _ => panic!("Cannot retrieve Rank for Value: {value}")
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Ranks::Ace => "Ace",
            Ranks::King => "King",
            Ranks::Queen => "Queen",
            Ranks::Jack => "Jack",
            Ranks::Ten => "10",
            Ranks::Nine => "9",
            Ranks::Eight => "8",
            Ranks::Seven => "7",
            Ranks::Six => "6",
            Ranks::Five => "5",
            Ranks::Four => "4",
            Ranks::Three => "3",
            Ranks::Two => "2",
            Ranks::Blank => "No Rank Set.",
        }
    }
}
