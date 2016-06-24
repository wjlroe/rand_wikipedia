extern crate hyper;
extern crate scraper;
extern crate rand;
extern crate clap;

use clap::{Arg, App};
use std::io::Read;
use hyper::Client;
use scraper::{Html, Selector};

const COMPOSERS_BY_NAME: &'static str = "https://en.wikipedia.org/wiki/List_of_composers_by_name";
const LIST_ITEMS: &'static str = "#bodyContent > #mw-content-text > div > ul > li > a";

fn rand_item_from_page(page: &str) -> String {
    let client = Client::new();
    let mut res = client.get(page).send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    let mut body_string = String::new();
    res.read_to_string(&mut body_string).unwrap();
    let document = Html::parse_document(&body_string);
    let selector = Selector::parse(LIST_ITEMS).unwrap();

    let mut rng = rand::thread_rng();
    let names = document.select(&selector)
        .map(|element| element.text().collect::<Vec<&str>>().join(" "));
    let samples = rand::sample(&mut rng, names, 1);
    let element: &String = samples.first().unwrap();
    element.clone()
}

fn main() {
    let cli = App::new("Random wikipedia thing")
        .version("0.1")
        .arg(Arg::with_name("page")
            .short("p")
            .long("page")
            .value_name("PAGE")
            .help("Which page to scrape")
            .required(true)
            .takes_value(true))
        .get_matches();

    let cmd = cli.value_of("page").unwrap_or("composers");
    let page = match cmd {
        "composers" => Some(COMPOSERS_BY_NAME),
        _ => None,
    };
    if let Some(page_name) = page {
        println!("{}", rand_item_from_page(page_name));
    } else {
        println!("'{}' not understood", cmd);
    }
}
