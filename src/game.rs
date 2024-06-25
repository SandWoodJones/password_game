use rand::seq::SliceRandom;
use moontool::moon::MoonPhase;
use std::collections::HashSet;

mod wordle;
mod geoguess;

#[cfg(test)]
mod tests;

use crate::roman;
use wordle::Wordle;
use geoguess::GeoGuess;

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
    Include2LetterPeriodic,
    IncludeMoonPhase,
    IncludeCountryName,
    IncludeLeapYear,
    IncludeBestMove
}

pub struct PasswordGame {
    current_rule: GameRules,
    generated_captcha: String,
    todays_wordle: Wordle,
    current_moon_phase: MoonPhase,
    geoguesser: GeoGuess
}

impl Default for PasswordGame {
   fn default() -> Self {
        static VALID_CAPTCHA: [&str; 14] = ["be3bp", "74eyg", "x4gg5", "p2m6n", "pcede", "bdg84", "52447",
                                            "y4n6m", "y5w28", "mgw3n", "cen55", "y4n6m", "wce5n", "d22bd"];
        
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
            Include2LetterPeriodic => { Self::includes_2_letter_periodic_symbol(pass) },
            IncludeMoonPhase => { self.includes_moon_phase_emoji(pass) },
            IncludeCountryName => { pass.to_lowercase().contains(&self.geoguesser.answer) },
            IncludeLeapYear => { Self::includes_leap_year(pass) },
            IncludeBestMove => { self.includes_best_chess_move(pass) }
        }
    }
}

impl PasswordGame {
    fn adds_to_25(s: &str) -> bool {
        let mut r = 0;

        for c in s.chars() {
            if let Some(d) = c.to_digit(10) { r += d; }
        }

        r >= 25
    }

    fn includes_month(s: &str) -> bool {
        static MONTHS: [&str; 12] = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];
        let s = s.to_lowercase();

        MONTHS.iter().any(|m| s.contains(m))
    }

    fn includes_sponsors(s: &str) -> bool {
        let sponsors = ["pepsi", "starbucks", "shell"];
        let s = s.to_lowercase();

        sponsors.iter().any(|m| s.contains(m))
    }

    fn roman_numerals_multiply_35(s: &str) -> bool {
        let s: String = s.chars().filter(|c| ['I', 'V', 'X', 'L', 'C', 'D', 'M'].contains(c) ).collect();

        let product = roman::roman_to_int(&s).expect("string is filtered so Err is unreachable")
                        .iter().try_fold(1usize, |acc, &num| acc.checked_mul(num as usize));

        if let Some(n) = product { n >= 35 }
        else { false }
    }

    fn includes_2_letter_periodic_symbol(s: &str) -> bool {
        static PERIODIC_TABLE: [&str; 104] = [
                                                                                                                  "He",
            "Li", "Be",                                                                                           "Ne",
            "Na", "Mg",                                                             "Al", "Si",             "Cl", "Ar",
                  "Ca", "Sc", "Ti",       "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As", "Se", "Br", "Kr",
            "Rb", "Sr",       "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In", "Sn", "Sb", "Te",       "Xe",
            "Cs", "Ba",       "Hf", "Ta",       "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi", "Po", "At", "Rn",
            "Fr", "Ra",       "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh", "Fl", "Mc", "Lv", "Ts", "Og",
                              "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu",
                              "Ac", "Th", "Pa",       "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md", "No", "Lr"
        ];

        let symbols = HashSet::from(PERIODIC_TABLE);
        
        for w in s.chars().collect::<Vec<char>>().windows(2) {
            if symbols.contains(w.iter().collect::<String>().as_str()) { return true; }
        }

        false
    }

    fn includes_moon_phase_emoji(&self, s: &str) -> bool {
        match self.current_moon_phase.phase_name.as_str() {
            "New" => { s.contains('\u{1F311}') },
            "Waxing Crescent" | "Waning Crescent" => { s.contains('\u{1F312}') || s.contains('\u{1F318}') },
            "First Quarter" | "Last Quarter" => { s.contains('\u{1F313}') || s.contains('\u{1F317}') },
            "Waxing Gibbous" | "Waning Gibbous" => { s.contains('\u{1F314}')},
            "Full" => { s.contains('\u{1F315}') },
            _ => unreachable!("error with moon phase library")
        }
    }

    fn includes_leap_year(s: &str) -> bool {
        fn is_leap(n: &[u32]) -> bool {
            if n.is_empty() { return false; }
            let n = n.iter().fold(0, |acc, elem| acc * 10 + elem);

            (n % 4 == 0) && ((n % 100 != 0) || (n % 400 == 0))
        }
        
        let mut number = Vec::new();
        for ch in s.chars() {
            if let Some(n) = ch.to_digit(10) { number.push(n) }
            else {
                if is_leap(&number) { return true; }
                number.clear();
            }
        }

        is_leap(&number)
    }

    fn includes_best_chess_move(&self, s: &str) -> bool {
        // https://neal.fun/password-game/chess/puzzle192.svg
        true
    }
}


