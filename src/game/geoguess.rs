use serde::Deserialize;
use rand::seq::SliceRandom;
use build_html::{Html, HtmlPage, HtmlContainer, Container, ContainerType};
use std::{io::Write, path::Path, boxed::Box};

pub(super) struct GeoGuess {
    pub answer: String,
    temp_file_path: Box<Path>
}

#[derive(Debug, Deserialize)]
struct GeoguessLocation {
    title: String,
    embed: String
}

impl Default for GeoGuess {
    fn default() -> Self {
        let mut temp_html_file = tempfile::Builder::new()
                    .prefix("password-game-temp")
                    .suffix(".html")
                    .tempfile()
                    .expect("error when creating geoguesser context");

        let possible_locations: Vec<GeoguessLocation> = serde_json::from_str(include_str!("possible_geoguesses.json")).expect("error parsing geoguess location");
        let picked_location = possible_locations.choose(&mut rand::thread_rng()).expect("safe to unwrap as array is not empty");

        let html = HtmlPage::new()
                        .with_style(concat!(
                            "html, body { height: 100%; }",
                            ".geo-wrapper {",
                                "width: 100%;",
                                "height: 100%;",
                                "overflow: hidden;",
                                "border-radius: 15px;",
                            "}",
                            ".geo { margin-top: -80px; }"
                        ))
                        .with_container(
                            Container::new(ContainerType::Div)
                                .with_attributes(vec![("class", "geo-wrapper")])
                                .with_html(Iframe::new("geo", format!("https://www.google.com/maps/embed?pb=!4v1686332628469!6m8!1m7!1s{}", picked_location.embed), "100%", "100%"))
                        )
                        .to_html_string();
                

        writeln!(temp_html_file, "{html}").unwrap();

        GeoGuess {
            answer: picked_location.title.to_lowercase(),
            temp_file_path: Box::from(temp_html_file.path())
        }
    }
}

#[derive(Debug)]
struct Iframe {
    class: String,
    src: String,
    width: String,
    height: String
}

impl Iframe {
    pub fn new(class: impl ToString, src: impl ToString, width: impl ToString, height: impl ToString) -> Self {
        Iframe {
            class: class.to_string(),
            src: src.to_string(),
            width: width.to_string(),
            height: height.to_string(),
        }
    }
}

impl Html for Iframe {
    fn to_html_string(&self) -> String {
        format!(r#"<iframe class="{}" src="{}" width="{}" height="{}"/>"#, self.class, self.src, self.width, self.height)
    }
}
