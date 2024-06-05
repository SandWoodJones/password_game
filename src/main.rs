mod game;

use game::{GameRules, PasswordGame};

fn main() {
    let mut game = PasswordGame::new();

    game.set_rule(GameRules::IncludeSponsors);
    game.play("MarchPepsi");
}
