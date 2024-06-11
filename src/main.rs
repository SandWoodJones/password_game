mod game;
mod roman;

use game::{GameRules, PasswordGame};

fn main() {
    let mut game = PasswordGame::new();

    game.set_rule(GameRules::IncludeLeapYear);
    game.play("asdasda1312daadsdaleap1601sdas");
}


