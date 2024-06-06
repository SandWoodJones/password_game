use rand::seq::SliceRandom;
use std::collections::HashSet;

mod wordle;

use crate::roman;
use wordle::Wordle;

#[derive(Debug, Default, Copy, Clone)]
pub enum GameRules {
    #[default]
    FiveChars = 0,
    IncludeNum,
    IncludeUpper,
    IncludeSpecial,
    AddTo25,
    IncludeMonth,
    IncludeRoman,
    IncludeSponsors,
    RomanMultiply35,
    IncludeCAPTCHA,
    IncludeWordle,
    Include2LetterPeriodic
}

pub struct PasswordGame {
    current_rule: GameRules,
    generated_captcha: String,
    todays_wordle: Wordle
}

impl Default for PasswordGame {
   fn default() -> Self {
        let captcha = ["be3bp", "74eyg", "x4gg5", "p2m6n", "pcede", "bdg84", "52447",
                       "y4n6m", "y5w28", "mgw3n", "cen55", "y4n6m", "wce5n", "d22bd"].choose(&mut rand::thread_rng()).expect("Safe to unwrap as array is not empty");

        PasswordGame {
            current_rule: Default::default(),
            generated_captcha: captcha.to_string(),
            todays_wordle: Default::default()
        }
    }
}

impl PasswordGame {
    pub fn new() -> Self { Default::default() }
    pub fn play(&self, pass: &str) { println!("{}", self.validate_password(pass, self.current_rule)); }

    pub fn set_rule(&mut self, rule: GameRules) { self.current_rule = rule; }

    fn validate_password(&self, pass: &str, rule: GameRules) -> bool {
        use GameRules::*;

        match rule {
            FiveChars => { pass.len() >= 5 },
            IncludeNum => { pass.chars().any(|c| c.is_ascii_digit() ) },
            IncludeUpper => { pass.chars().any(|c| c.is_uppercase() ) },
            IncludeSpecial => { pass.chars().any(|c| !c.is_ascii_alphanumeric()) },
            AddTo25 => { Self::adds_to_25(pass) },
            IncludeMonth => { Self::includes_month(pass) },
            IncludeRoman => { pass.chars().any(|c| ['I', 'V', 'X', 'L', 'C', 'D', 'M'].contains(&c) ) },
            IncludeSponsors => { Self::includes_sponsors(pass) },
            RomanMultiply35 => { Self::roman_numerals_multiply_35(pass) },
            IncludeCAPTCHA => { pass.contains(&self.generated_captcha) },
            IncludeWordle => { pass.contains(&self.todays_wordle.solution) },
            Include2LetterPeriodic => { Self::includes_2_letter_periodic_symbol(pass) }
        }
    }
}

impl PasswordGame {
    fn adds_to_25(s: &str) -> bool {
        let mut r = 0;

        for c in s.chars() {
            if let Some(d) = c.to_digit(10) { r += d; }
        }

        return r >= 25;
    }

    fn includes_month(s: &str) -> bool {
        let months = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];
        let s = s.to_lowercase();

        months.iter().any(|m| s.contains(m))
    }

    fn includes_sponsors(s: &str) -> bool {
        let sponsors = ["pepsi", "starbucks", "shell"];
        let s = s.to_lowercase();

        sponsors.iter().any(|m| s.contains(m))
    }

    fn roman_numerals_multiply_35(s: &str) -> bool {
        let s: String = s.chars().filter(|c| ['I', 'V', 'X', 'L', 'C', 'D', 'M'].contains(&c) ).collect();

        let product = roman::roman_to_int(&s).expect("string is filtered so Err is unreachable")
                        .iter().try_fold(1usize, |acc, &num| acc.checked_mul(num as usize));

        if let Some(n) = product { n >= 35 }
        else { false }
    }

    fn includes_2_letter_periodic_symbol(s: &str) -> bool {
        let symbols = HashSet::from([
                                                                                                                  "he",
            "li", "be",                                                                                           "ne",
            "na", "mg",                                                             "al", "si",             "cl", "ar",
                  "ca", "sc", "ti",       "cr", "mn", "fe", "co", "ni", "cu", "zn", "ga", "ge", "as", "se", "br", "kr",
            "rb", "sr",       "zr", "nb", "mo", "tc", "ru", "rh", "pd", "ag", "cd", "in", "sn", "sb", "te",       "xe",
            "cs", "ba",       "hf", "ta",       "re", "os", "ir", "pt", "au", "hg", "tl", "pb", "bi", "po", "at", "rn",
            "fr", "ra",       "rf", "db", "sg", "bh", "hs", "mt", "ds", "rg", "cn", "nh", "fl", "mc", "lv", "ts", "og",
                              "la", "ce", "pr", "nd", "pm", "sm", "eu", "gd", "tb", "dy", "ho", "er", "tm", "yb", "lu",
                              "ac", "th", "pa",       "np", "pu", "am", "cm", "bk", "cf", "es", "fm", "md", "no", "lr"
        ]);
        
        let s = s.to_lowercase();

        for w in s.chars().collect::<Vec<char>>().windows(2) {
            if symbols.contains(w.iter().collect::<String>().as_str()) { return true; }
        }

        false
    }
}
