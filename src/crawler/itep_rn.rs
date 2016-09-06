use std::io::Read;
use std::collections::HashMap;

use models::the_dead::TheDead;

use hyper;

use select::document::Document;
use select::predicate::*;

use encoding::{Encoding, DecoderTrap, EncoderTrap};
use encoding::all::WINDOWS_1252;
use encoding::all::UTF_8;

pub fn request() -> Vec<TheDead> {
    let client = hyper::Client::new();
    let url = "http://www2.defesasocial.rn.gov.br/estatistica/index.php";
    let mut res = match client.get(url).send() {
        Ok(request) => request,
        Err(_) => panic!("Something wrong with request.")
    };
    let mut s = Vec::new();
    match res.read_to_end(&mut s) {
        Ok(s) => s,
        Err(_) => panic!("Something wrong with response.")
    };
    let res_decoded = WINDOWS_1252.decode(&s, DecoderTrap::Strict);
    let res_utf = UTF_8.encode(&res_decoded.unwrap(), EncoderTrap::Strict);

    let body = String::from_utf8(res_utf.unwrap());
    let document = Document::from_str(&body.unwrap());
    let header = ["register_date", "deceased_date", "name", "gender", "age", "address", "place",
    "cause_death", "city"];
    let deads = document.find(Name("tr")).iter().filter(|tr| tr.find(Name("td")).first()
    .and_then(|e| Some(e.text().trim().to_owned()))
    .unwrap_or("".to_owned()) != "Data/Entrada".to_string())
        .fold(<Vec<TheDead>>::new(), |mut acc, el| {
            let the_dead = el.find(Name("td")).iter().enumerate()
                .fold(HashMap::new(), |mut dead, (i, item)| {
                    dead.insert(header[i], item.text().trim().to_owned());
                    return dead;
                });

            acc.push(TheDead {
                id: 0,
                register_date: the_dead.get("register_date").unwrap().clone(),
                deceased_date: the_dead.get("deceased_date").unwrap().clone(),
                name: the_dead.get("name").unwrap().clone(),
                gender: the_dead.get("gender").unwrap().clone(),
                age: the_dead.get("age").unwrap().clone(),
                address: the_dead.get("address").unwrap().clone(),
                place: the_dead.get("place").unwrap().clone(),
                cause_death: the_dead.get("cause_death").unwrap().clone(),
                city: the_dead.get("city").unwrap().clone(),
                source: "itep-rn".to_owned()
            });
            return acc;
        });
    return deads;
}
