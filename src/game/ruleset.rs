use crate::constants::*;
use super::PasswordGame;
use super::roman;

#[derive(Debug, Default, Copy, Clone)]
pub enum Rule {
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
    IncludeBestMove,
    IncludePaul,
    AtomicAddTo200,
    AllVowelsBolded,
    Fire,
    StrongEnough,
    IncludeAffirmations
}

impl Rule {
    pub fn validate(&self, game: &PasswordGame, s: &str) -> bool {
        use Rule::*;
        
        match self {
            FiveChars              => { s.len() >= 5 },
            IncludeNum             => { s.chars().any(|c| c.is_ascii_digit() ) },
            IncludeUpper           => { s.chars().any(|c| c.is_uppercase() ) },
            IncludeSpecial         => { s.chars().any(|c| !c.is_ascii_alphanumeric()) },
            AddTo25                => { Self::adds_to_25(s) },
            IncludeMonth           => { Self::includes_month(s) },
            IncludeRoman           => { s.chars().any(|c| ['I', 'V', 'X', 'L', 'C', 'D', 'M'].contains(&c) ) },
            IncludeSponsors        => { Self::includes_sponsors(s) },
            RomanMultiply35        => { Self::roman_numerals_multiply_35(s) },
            IncludeCAPTCHA         => { s.contains(&game.generated_captcha) },
            IncludeWordle          => { s.contains(&game.todays_wordle.solution) },
            Include2LetterPeriodic => { Self::includes_2_letter_periodic_symbol(s) },
            IncludeMoonPhase       => { Self::includes_moon_phase_emoji(game, s) },
            IncludeCountryName     => { s.to_lowercase().contains(&game.geoguesser.answer) },
            IncludeLeapYear        => { Self::includes_leap_year(s) },
            IncludeBestMove        => { Self::includes_best_chess_move(game, s) },
            IncludePaul            => { Self::includes_chicken_egg(game, s) },
            AtomicAddTo200         => { Self::atomic_numbers_add_to_200(s) },
            AllVowelsBolded        => { todo!() },
            Fire                   => { todo!() },
            StrongEnough           => { s.chars().filter(|&c| c == '\u{1f3cb}').count() >= 3 },
            IncludeAffirmations    => { Self::includes_positive_affirmations(s) }
        }
    }
}

impl Rule {
    fn adds_to_25(s: &str) -> bool {
        let mut r = 0;

        for c in s.chars() {
            if let Some(d) = c.to_digit(10) { r += d; }
        }

        r == 25
    }

    fn includes_month(s: &str) -> bool {
        let s = s.to_lowercase();

        MONTHS.iter().any(|m| s.contains(m))
    }

    fn includes_sponsors(s: &str) -> bool {
        let s = s.to_lowercase();

        SPONSORS.iter().any(|m| s.contains(m))
    }

    fn roman_numerals_multiply_35(s: &str) -> bool {
        let s: String = s.chars().filter(|c| ['I', 'V', 'X', 'L', 'C', 'D', 'M'].contains(c) ).collect();

        let product = roman::roman_to_int(&s)
                        .expect("string is filtered so Err is unreachable")
                        .iter().try_fold(1usize, |acc, &num| acc.checked_mul(num as usize));

        if let Some(n) = product { n >= 35 }
        else { false }
    }

    fn includes_2_letter_periodic_symbol(s: &str) -> bool {
        let symbols: Vec<_> = PERIODIC_TABLE.iter()
                                    .filter(|sym| sym.len() == 2)
                                    .collect();

        symbols.iter().any(|&sym| s.contains(sym))
    }

    fn includes_moon_phase_emoji(game: &PasswordGame, s: &str) -> bool {
        match game.current_moon_phase.phase_name.as_str() {
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

    fn includes_best_chess_move(game: &PasswordGame, s: &str) -> bool {
        // https://neal.fun/password-game/chess/puzzle192.svg
        todo!()
    }

    fn includes_chicken_egg(game: &PasswordGame, s: &str) -> bool {
        todo!()
    }

    fn atomic_numbers_add_to_200(s: &str) -> bool {
        let mut sum = 0;
        
        let mut s = s.chars().peekable();
        while let Some(c) = s.next() {
            if c.is_ascii_uppercase() {
                let mut symbol = String::from(c);
                if let Some(lower) = s.next_if(|&x| x.is_ascii_lowercase()) { symbol.push(lower); }
                
                if let Some(idx) = PERIODIC_TABLE.iter().position(|&x| x == symbol) { sum += idx + 1; }
            }
        }

        sum == 200
    }

    fn includes_positive_affirmations(s: &str) -> bool {
        let mut s = s.to_lowercase();
        s.retain(|c| !c.is_whitespace());

        AFFIRMATIONS.iter().any(|m| s.contains(m))
    }
}
