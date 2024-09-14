use crate::cards::{Card, Suit};

pub mod cards;

pub enum GameState {
    Waiting,
    Bidding,
    Playing,
}

pub struct Player {
    name: String,
    hand: Vec<Card>,
    bid: Option<u8>,
    sets: u8,
    score: u32,
    card: Option<Card>,
}

pub struct Game {
    state: GameState,
    players: Vec<Player>,
    current_round: u8,
    current_player: u8,
    trump: Option<Suit>,
    deck: Vec<Card>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::Waiting,
            players: vec![],
            current_round: 0,
            current_player: 0,
            trump: None,
            deck: vec![],
        }
    }

    pub fn add_player(&mut self, player_name: String) -> u8 {
        self.players.push(Player {
            name: player_name,
            hand: vec![],
            bid: None,
            sets: 0,
            score: 0,
            card: None,
        });
        self.players.len() as u8 - 1
    }

    pub fn start_next_round(&mut self) {
        self.deck = cards::gen_deck(4);
        self.current_round += 1;

        assert!(self.deck.len() >= self.current_round as usize * self.players.len());

        self.players.iter_mut().for_each(|p| {
            p.hand = self.deck.drain(..self.current_round as usize).collect();
            p.bid = None;
        });

        self.trump = Some(match self.deck.first() {
            Some(Card::Regular { suit, .. }) => *suit,
            _ => Suit::Heart,
        });

        self.state = GameState::Bidding;
    }

    pub fn make_bid(&mut self, player: u8, bid: u8) {
        assert_eq!(self.current_player, player);
        assert!(bid <= self.current_round);
        assert!(matches!(self.state, GameState::Bidding));

        self.players[self.current_player as usize].bid = Some(bid);

        self.current_player = (self.current_player + 1) % self.players.len() as u8;

        if self.players.iter().all(|p| p.bid.is_some()) {
            self.state = GameState::Playing;
        }
    }

    pub fn play_card(&mut self, player: u8, card: Card) {
        assert_eq!(self.current_player, player);
        assert!(self.players[player as usize].hand.contains(&card));
        assert!(matches!(self.state, GameState::Playing));

        let card_index = self.players[player as usize]
            .hand
            .iter()
            .position(|c| *c == card)
            .unwrap();

        self.players[player as usize].card = Some(card);
        self.players[player as usize].hand.swap_remove(card_index);

        self.current_player = (self.current_player + 1) % self.players.len() as u8;

        if self.players.iter().all(|p| p.card.is_some()) {
            // Who gets the set?
        }
    }

    pub fn trump(&self) -> Option<Suit> {
        self.trump
    }

    pub fn hand(&self, player: u8) -> &[Card] {
        &self.players[player as usize].hand
    }
}
