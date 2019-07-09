// Inspired by <https://www.theyfightcrime.org>
use actix_web::{web, App, HttpServer};
use clap::Arg;
use rand::seq::SliceRandom;
use serde::Deserialize;

use std::io::{Error, Result};
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Deserialize)]
struct TheyFightCrime {
    adjectives: Vec<String>,
    nouns: Vec<String>,
    with: Vec<String>,
}

impl TheyFightCrime {
    fn load(path: &Path) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        serde_json::from_reader(file).map_err(Error::from)
    }

    fn generate(&self) -> String {
        let rng = &mut rand::thread_rng();
        let (adjective1, adjective2) = choose_two(&self.adjectives, rng);
        let (noun1, noun2) = choose_two(&self.nouns, rng);
        let (description1, description2) = choose_two(&self.with, rng);

        format!(
            "One's {} {} with {}, the other's {} {} with {}. Together, they fight crime!",
            adjective1, noun1, description1, adjective2, noun2, description2
        )
    }
}

fn choose_two<'a, R: rand::Rng>(slice: &'a [String], rng: &mut R) -> (&'a String, &'a String) {
    let chosen: Vec<_> = slice.choose_multiple(rng, 2).collect();

    match chosen.as_slice() {
        [one, two] => (one, two),
        _ => panic!("Expected two values in {:?}", chosen),
    }
}

fn they_fight_crime(data: web::Data<TheyFightCrime>) -> String {
    data.generate()
}

fn main() -> Result<()> {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("8080"),
        )
        .arg(
            Arg::with_name("data_path")
                .short("d")
                .long("data-path")
                .takes_value(true)
                .default_value("./data.json"),
        )
        .get_matches();

    let port = matches
        .value_of("port")
        .map(u16::from_str)
        .expect("a value for port")
        .expect("a valid port number");
    let data_path = matches
        .value_of("data_path")
        .map(Path::new)
        .expect("a value for data_path");

    let tfc_data = TheyFightCrime::load(data_path)?;
    let web_data = web::Data::new(tfc_data);

    let server = HttpServer::new(move || {
        App::new()
            .register_data(web_data.clone())
            .route("/", web::get().to(they_fight_crime))
    })
    .bind(format!("0.0.0.0:{}", port))?;

    println!("Listening on port {}", port);
    server.run()
}
