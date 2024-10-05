use rand::thread_rng;
use wizard::GameState;

fn main() {
    let mut game = GameState::new(4);

    let mut rng = thread_rng();

    game.start_round(&mut rng);
}
