use rand::{seq::SliceRandom, Rng};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Club, Suit::Spade, Suit::Heart, Suit::Diamond];
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub const ALL: [Rank; 13] = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Card {
    Standard { suit: Suit, rank: Rank },
    Joker,
}

pub fn gen_wizard_deck(rng: &mut impl Rng) -> Vec<Card> {
    let mut out = Vec::with_capacity(56);

    for suit in Suit::ALL {
        for rank in Rank::ALL {
            out.push(Card::Standard { suit, rank });
        }
    }

    for _ in 0..4 {
        out.push(Card::Joker);
    }

    out.shuffle(rng);

    out
}
