mod constants;
mod game;

use game::{PasswordGame, ruleset::Rule};

fn main() {
    let mut game = PasswordGame::new();

    game.set_rule(Rule::AtomicAddTo200);
    game.play("Og  AuHeHH");
}


