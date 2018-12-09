extern crate csv;
extern crate rand;
extern crate reqwest;
extern crate select;
#[macro_use]
extern crate serde_derive;

use csv::WriterBuilder;
use rand::{thread_rng, Rng};
use select::document::Document;
#[allow(unused_imports)]
use select::predicate::{Class, Name, Predicate};
use std::error::Error;
use std::{thread, time};

#[derive(Serialize)]
#[derive(Debug)]
struct Label <'a> {
    label_id: u64,
    name: &'a str,
    profile: &'a  str,
    // pratice Option, Vec, Match types
    // contact_info: &'a str,
    // sublabels: Vec<String>,
    // sites: &'a str,
}

fn main()-> Result<(), Box<Error>> {
    let file_path = std::path::Path::new("labels.csv");
    let mut wtr = WriterBuilder::new()
        .delimiter(b',')
        .from_path(file_path)?;

    for i in 1204..1218 {
        let wait_period: u64 = thread_rng().gen_range(150, 350);
        thread::sleep(time::Duration::from_millis(wait_period));
        let mut url = String::from(format!("https://www.discogs.com/label/{}", i));
        let req = reqwest::get(&url).unwrap();
        if req.status().is_success() {
            let document = Document::from_read(req).unwrap();
            let label_id: u64 = i;
            for node in document.find(Class("profile")) {
                wtr.serialize(Label {
                    label_id: label_id,
                    name: &node.find(Name("h1")).next().unwrap().text(),
                    // practic string methods:
                    profile: &node.find(Class("content")).next().unwrap().text().replacen('\n', "", 10).trim(),
                    // sublabels: node.find(Class("content")).take(1).map(|txt| txt.text()).take(3).collect(),
                    // contact_info: &node.find(Class("content")).nth(2).unwrap().text(),                 
                    // sites: &node.find(Class("content")).last().unwrap().text(),
                })?;
            };
        } else {
            continue;
        }
    }

    Ok(())
}