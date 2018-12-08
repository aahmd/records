extern crate csv;
extern crate rand;
extern crate reqwest;
extern crate select;
#[macro_use]
extern crate serde_derive;

// use csv::WriterBuilder;
use rand::{thread_rng, Rng};
use select::document::Document;
#[allow(unused_imports)]
use select::predicate::{Class, Name, Predicate};
use std::error::Error;
use std::{thread, time};

#[derive(Serialize)]
#[derive(Debug)]
struct Label <'a> {
    name: &'a str,
    label_id: u64,
    // pratice Option and Match types
    profile: Option<&'a str>,
    sublabels: &'a str,
    // contact_info: &'a str,
    // sites: &'a str,
}

fn main()-> Result<(), Box<Error>> {
    let file_path = std::path::Path::new("test.csv");
    let mut wtr = csv::Writer::from_path(file_path).unwrap();
    // use non-default csv writer
    // let mut wtr = WriterBuilder::new()
    //     .delimiter(b',')
    //     .escape(b'\n')
    //     .from_path(file_path);

    for i in 1215..1220 {
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
                    profile: node.find(Class("content")).next().unwrap().retain(),
                    sublabels: &node.find(Class("content")).next().unwrap().text(),
                    // contact_info:
                    // sites:
                })?;
            };
        } else {
            continue;
        }
    }

    Ok(())
}