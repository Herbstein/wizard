use rand::Rng;

use crate::{
    cards,
    cards::{Card, Suit},
};

struct PlayerState {
    hand: Vec<Card>,
    bid: Option<u32>,
    tricks_won: u32,
    score: i32,
}

struct TrickState {
    trick_trump: Option<Suit>,
    cards_played: Vec<(usize, Card)>,
}

impl TrickState {
    fn find_player_of_highest_rank_in_trump_suit(&self, trump: Suit) -> Option<usize> {
        self.cards_played
            .iter()
            .filter_map(|(player, card)| match card {
                Card::Standard { suit, rank } if *suit == trump => Some((player, rank)),
                _ => None,
            })
            .max_by_key(|(_, rank)| *rank)
            .map(|(player, _)| *player)
    }
}

pub struct GameState {
    deck: Vec<Card>,
    players: Vec<PlayerState>,
    trump_suit: Option<Suit>,
    current_round: u32,
    dealer_index: usize,
    player_index: usize,
    current_trick: Option<TrickState>,
}

#[derive(Debug)]
pub enum PlayCardError {
    PlayerDoesNotHaveCard,
    PlayerDoesNotExist,
    NoCurrentTrick,
}

#[derive(Debug)]
pub enum EndTrickError {
    TrickNotStarted,
    TrumpSuitNotSelected,
    NotAllPlayersHavePlayed,
    NoTrickTrumpSet,
    PlayerDoesNotExist,
    NoWinnerFound,
}

pub enum EndRoundError {
    PlayerDoesNotExist,
}

impl GameState {
    pub fn new(num_players: usize) -> Self {
        let mut players = Vec::new();
        for _ in 0..num_players {
            players.push(PlayerState {
                hand: Vec::new(),
                bid: None,
                tricks_won: 0,
                score: 0,
            });
        }

        GameState {
            deck: Vec::new(),
            players,
            trump_suit: None,
            current_round: 0,
            dealer_index: 0,
            player_index: 1,
            current_trick: None,
        }
    }

    pub fn start_round(&mut self, rng: &mut impl Rng) -> bool {
        self.current_round += 1;

        self.dealer_index = (self.dealer_index + 1) % self.players.len();
        self.player_index = (self.dealer_index + 1) % self.players.len();

        let deck = cards::gen_wizard_deck(rng);

        if self.players.len() * self.current_round as usize > deck.len() {
            return false;
        }

        for (i, player) in self.players.iter_mut().enumerate() {
            player.bid = None;
            player.hand =
                deck[i * self.current_round as usize..][..self.current_round as usize].to_vec();
        }

        self.trump_suit = Some(
            match deck.get(self.players.len() * self.current_round as usize) {
                Some(Card::Standard { suit, .. }) => *suit,
                _ => Suit::Heart,
            },
        );

        self.deck = deck
            .get(self.players.len() * self.current_round as usize + 1..)
            .unwrap_or_default()
            .to_vec();

        true
    }

    pub fn place_bid(&mut self, bid: u32) {
        if let Some(player) = self.players.get_mut(self.player_index) {
            player.bid = Some(bid);

            self.player_index = (self.player_index + 1) % self.players.len();
        }
    }

    pub fn play_card(&mut self, card: Card) -> Result<(), PlayCardError> {
        let Some(player) = self.players.get_mut(self.player_index) else {
            return Err(PlayCardError::PlayerDoesNotExist);
        };

        let Some(card_index) = player.hand.iter().position(|c| *c == card) else {
            return Err(PlayCardError::PlayerDoesNotHaveCard);
        };

        let Some(trick) = &mut self.current_trick else {
            return Err(PlayCardError::NoCurrentTrick);
        };

        player.hand.remove(card_index);
        trick.cards_played.push((self.player_index, card));

        if trick.trick_trump.is_none() {
            trick.trick_trump = Some(match card {
                Card::Standard { suit, .. } => suit,
                Card::Joker => Suit::Heart,
            })
        }

        self.player_index = (self.player_index + 1) % self.players.len();

        Ok(())
    }

    pub fn start_trick(&mut self) {
        self.current_trick = Some(TrickState {
            trick_trump: None,
            cards_played: vec![],
        });
    }

    pub fn end_trick(&mut self) -> Result<(), EndTrickError> {
        let Some(trick) = std::mem::take(&mut self.current_trick) else {
            return Err(EndTrickError::TrickNotStarted);
        };

        let Some(trick_trump) = trick.trick_trump else {
            return Err(EndTrickError::NoTrickTrumpSet);
        };

        if trick.cards_played.len() != self.players.len() {
            return Err(EndTrickError::NotAllPlayersHavePlayed);
        }

        let Some(trump_suit) = self.trump_suit else {
            return Err(EndTrickError::TrumpSuitNotSelected);
        };

        let winner = trick
            .cards_played
            .iter()
            .filter(|(_, card)| matches!(card, Card::Joker))
            .map(|(player, _)| *player)
            .next();

        let winner = winner
            .or_else(|| trick.find_player_of_highest_rank_in_trump_suit(trump_suit))
            .or_else(|| trick.find_player_of_highest_rank_in_trump_suit(trick_trump));

        if let Some(winner) = winner {
            let Some(player) = self.players.get_mut(winner) else {
                return Err(EndTrickError::PlayerDoesNotExist);
            };
            player.tricks_won += 1;

            self.player_index = winner;

            self.current_trick = None;

            Ok(())
        } else {
            Err(EndTrickError::NoWinnerFound)
        }
    }

    pub fn end_round(&mut self) -> Result<(), EndRoundError> {
        for player in self.players.iter_mut() {
            let Some(bid) = player.bid else {
                return Err(EndRoundError::PlayerDoesNotExist);
            };

            if player.tricks_won == bid {
                player.score += bid as i32 * 10 + 20;
            } else {
                player.score -= (player.tricks_won as i32 - bid as i32).abs() * 10;
            }

            player.tricks_won = 0;
        }

        self.trump_suit = None;

        Ok(())
    }
}

#[derive(Debug)]
pub struct PlayerRoundInfo {
    bid: Option<u32>,
    tricks_won: u32,
}

#[derive(Debug)]
pub struct RoundInfo {
    pub trump: Suit,
    pub players: Vec<PlayerRoundInfo>,
}

#[derive(Debug)]
pub struct TrickInfo {
    pub trump: Option<Suit>,
}

#[derive(Debug)]
pub struct PlayerInfo {
    pub hand: Vec<Card>,
    pub score: i32,
    pub bid: Option<u32>,
    pub tricks_won: u32,
}

impl GameState {
    pub fn round_info(&self) -> Option<RoundInfo> {
        let trump_suit = self.trump_suit?;

        Some(RoundInfo {
            trump: trump_suit,
            players: self
                .players
                .iter()
                .map(|p| PlayerRoundInfo {
                    bid: p.bid,
                    tricks_won: p.tricks_won,
                })
                .collect(),
        })
    }

    pub fn trick_info(&self) -> Option<TrickInfo> {
        let trick = self.current_trick.as_ref()?;

        Some(TrickInfo {
            trump: trick.trick_trump,
        })
    }

    pub fn current_player_info(&self) -> Option<PlayerInfo> {
        let player = self.players.get(self.player_index)?;

        Some(PlayerInfo {
            hand: player.hand.clone(),
            score: player.score,
            bid: player.bid,
            tricks_won: player.tricks_won,
        })
    }
}
