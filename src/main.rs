// Inspired by <https://www.theyfightcrime.org>
use actix_web::{web, App, HttpServer};
use rand::seq::SliceRandom;
use serde::Deserialize;
use structopt::StructOpt;

use std::io::{Error, Result};
use std::path::{Path, PathBuf};

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

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    #[structopt(long, short, default_value = "8080")]
    port: u16,
    #[structopt(long, short, default_value = "./data.json", parse(from_os_str))]
    data_path: PathBuf
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let tfc_data = TheyFightCrime::load(&options.data_path)?;
    let web_data = web::Data::new(tfc_data);

    let server = HttpServer::new(move || {
        App::new()
            .register_data(web_data.clone())
            .route("/", web::get().to(they_fight_crime))
    })
    .bind(format!("0.0.0.0:{}", options.port))?;

    println!("Listening on port {}", options.port);
    server.run()
}
