use std::{fmt, fmt::Write, str::FromStr};

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

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Suit::Spade => 'S',
            Suit::Heart => 'H',
            Suit::Diamond => 'D',
            Suit::Club => 'C',
        })
    }
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

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Card {
    Standard { suit: Suit, rank: Rank },
    Joker,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Card::Standard { suit, rank } => {
                write!(f, "{}{}", suit, rank)
            }
            Card::Joker => f.write_str("Joker"),
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "joker" => Ok(Self::Joker),
            _ => {
                let (suit, rank) = value.split_at(1);
                let suit = match suit {
                    "D" => Suit::Diamond,
                    "H" => Suit::Heart,
                    "C" => Suit::Club,
                    "S" => Suit::Spade,
                    _ => return Err(value.into()),
                };
                let rank = match rank {
                    "2" => Rank::Two,
                    "3" => Rank::Three,
                    "4" => Rank::Four,
                    "5" => Rank::Five,
                    "6" => Rank::Six,
                    "7" => Rank::Seven,
                    "8" => Rank::Eight,
                    "9" => Rank::Nine,
                    "10" => Rank::Ten,
                    "J" => Rank::Jack,
                    "Q" => Rank::Queen,
                    "K" => Rank::King,
                    "A" => Rank::Ace,
                    _ => return Err(value.into()),
                };

                Ok(Self::Standard { suit, rank })
            }
        }
    }
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
