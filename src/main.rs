use std::io::Write;

use rand::thread_rng;
use wizard::{cards::Card, game::GameState};

const NUM_PLAYERS: usize = 4;

fn main() {
    let mut game = GameState::new(NUM_PLAYERS);

    let mut rng = thread_rng();

    println!("Starting round...");
    game.start_round(&mut rng);

    let round_info = game.round_info().unwrap();
    println!("Current round:\n\tTrump: {}", round_info.trump);

    println!("Starting trick...");
    game.start_trick();

    println!("Players are now bidding...");
    for i in 0..NUM_PLAYERS {
        println!("Player {}...", i);
        let player_info = game.current_player_info().unwrap();
        println!(
            "\tHand: {}",
            player_info
                .hand
                .into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );

        let mut stdout = std::io::stdout();
        write!(stdout, "Bid: ").unwrap();
        stdout.flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        let bid = input.parse::<u32>().unwrap();

        game.place_bid(bid);
    }

    println!("Players are now playing...");
    for i in 0..NUM_PLAYERS {
        println!("Player {}...", i);
        let player_info = game.current_player_info().unwrap();
        println!(
            "\tHand: {}",
            player_info
                .hand
                .into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );

        let mut stdout = std::io::stdout();
        write!(stdout, "Play card: ").unwrap();
        stdout.flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        let card = input.parse::<Card>().unwrap();

        game.play_card(card).unwrap();
    }

    game.end_trick().unwrap();

    let new_round_info = game.round_info().unwrap();
    println!("{new_round_info:#?}")
}
