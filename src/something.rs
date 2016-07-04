extern crate hyper;
extern crate clap;

use std::fs::File;
use std::io::Write;
use clap::{Arg, App};
use std::io::Read;
use hyper::Client;

const COMPOSERS_BY_NAME: &'static str = "https://en.wikipedia.org/wiki/List_of_composers_by_name";

fn rand_item_from_page(page: &str) -> String {
    let client = Client::new();
    let mut res = client.get(page).send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    let mut body_string = String::new();
    let mut cached_file = File::create(page.to_string() + ".html").unwrap();
    res.read_to_string(&mut body_string).unwrap();
    cached_file.write_all(body_string.as_bytes()).unwrap();

    let names = parse_page(body_string);
    rand_name(names)
}

fn main() {
    let cli = App::new("Random wikipedia thing")
        .version("0.1")
        .arg(Arg::with_name("page")
            .short("p")
            .long("page")
            .value_name("PAGE")
            .help("Which page to scrape")
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
