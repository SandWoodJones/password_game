use chrono::offset::Local;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct Wordle {
    pub(super) solution: String
}

impl Default for Wordle {
    fn default() -> Self {
        let url = format!("https://www.nytimes.com/svc/wordle/v2/{}.json", Local::now().format("%Y-%m-%d"));
        match reqwest::blocking::get(url) {
            Ok(resp) => {
                match resp.json() {
                    Ok(w) => { w }
                    Err(e) => panic!("error parsing wordle answer: {}", e)
                }
            },
            Err(e) => panic!("error requesting today's wordle answer: {}", e.without_url())
        }
    }
}
