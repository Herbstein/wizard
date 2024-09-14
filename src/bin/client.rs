use wizard::Game;

fn main() {
    let mut game = Game::new();
    let timm = game.add_player("Timm".to_string());
    let casper = game.add_player("Casper".to_string());
    let jeppe = game.add_player("Jeppe".to_string());
    let rasmus = game.add_player("Rasmus".to_string());

    game.start_next_round();

    game.make_bid(timm, 0);
    game.make_bid(casper, 1);
    game.make_bid(jeppe, 0);
    game.make_bid(rasmus, 0);

    game.play_card(timm, game.hand(timm)[0]);
    game.play_card(casper, game.hand(casper)[0]);
    game.play_card(jeppe, game.hand(jeppe)[0]);
    game.play_card(rasmus, game.hand(rasmus)[0]);
}
