mod game;
mod roman;

use game::{GameRules, PasswordGame};

fn main() {
    let mut game = PasswordGame::new();

    game.set_rule(GameRules::AtomicAddTo200);
    game.play("Og  AuHeHH");
}


