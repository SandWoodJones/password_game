use rand::seq::SliceRandom;
use moontool::moon::MoonPhase;

pub mod ruleset; // TODO: unpub

mod roman;
mod wordle;
mod geoguess;

#[cfg(test)]
mod tests;

use crate::constants::*;
use ruleset::Rule;
use wordle::Wordle;
use geoguess::GeoGuess;

pub struct PasswordGame {
    current_rule: Rule,
    generated_captcha: String,
    todays_wordle: Wordle,
    current_moon_phase: MoonPhase,
    geoguesser: GeoGuess
}

impl Default for PasswordGame {
   fn default() -> Self {
        let captcha = VALID_CAPTCHA.choose(&mut rand::thread_rng()).expect("safe to unwrap as array is not empty");

        PasswordGame {
            current_rule: Default::default(),
            generated_captcha: captcha.to_string(),
            todays_wordle: Default::default(),
            current_moon_phase: MoonPhase::now(),
            geoguesser: Default::default()
        }
    }
}

impl PasswordGame {
    pub fn new() -> Self { Default::default() }
    pub fn play(&self, pass: &str) { println!("{}", self.validate_password(pass)); }

    pub fn set_rule(&mut self, rule: Rule) { self.current_rule = rule; }

    fn validate_password(&self, pass: &str) -> bool { self.current_rule.validate(&self, pass) }
}

