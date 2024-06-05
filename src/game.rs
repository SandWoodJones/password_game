#[derive(Debug, Default, Copy, Clone)]
pub enum GameRules {
    #[default]
    FiveChars = 0,
    IncludeNum = 1,
    IncludeUpper = 2,
    IncludeSpecial = 3,
    AddTo25 = 4,
    IncludeMonth = 5,
    IncludeRoman = 6,
    IncludeSponsors = 7
}

#[derive(Default)]
pub struct PasswordGame {
    current_rule: GameRules
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
            IncludeSponsors => { Self::includes_sponsors(pass) }
        }
    }

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
}
