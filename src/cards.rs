#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Club, Suit::Spade, Suit::Heart, Suit::Diamond];
}

#[derive(Copy, Clone, PartialEq, Eq)]
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Card {
    Joker,
    Regular { suit: Suit, rank: Rank },
}

pub fn gen_deck(jokers: usize) -> Vec<Card> {
    let mut out = Vec::with_capacity(52 + jokers);

    for suit in Suit::ALL {
        for rank in Rank::ALL {
            out.push(Card::Regular { suit, rank });
        }
    }

    for _ in 0..jokers {
        out.push(Card::Joker);
    }

    out
}
