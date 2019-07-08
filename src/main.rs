// Inspired by <https://www.theyfightcrime.org>
use actix_web::{web, App, HttpServer};
use rand::seq::SliceRandom;
use serde::Deserialize;

use std::path::Path;

#[derive(Clone, Deserialize)]
struct TheyFightCrime {
    adjectives: Vec<String>,
    nouns: Vec<String>,
    with: Vec<String>,
}

impl TheyFightCrime {
    fn load(path: &std::path::Path) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        Ok(serde_json::from_reader(file)?)
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

fn main() -> Result<(), std::io::Error> {
    let port = 3000;

    let tfc_data = TheyFightCrime::load(Path::new("./data.json"))?;
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
