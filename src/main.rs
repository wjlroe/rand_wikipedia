extern crate clap;
extern crate rand_wikipedia;
extern crate reqwest;

use clap::{App, Arg};
use std::error::Error;

const COMPOSERS_BY_NAME: &'static str = "https://en.wikipedia.\
                                         org/wiki/List_of_composers_by_name";

fn rand_item_from_page(page: &str) -> Result<String, Box<dyn Error>> {
    let body_string = reqwest::blocking::get(page)?.text()?;
    // let mut cached_file = File::create(page.to_string() + ".html").unwrap();
    // cached_file.write_all(body_string.as_bytes()).unwrap();

    let names = rand_wikipedia::parse_page(body_string);
    Ok(rand_wikipedia::rand_name(&names))
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = App::new("Random wikipedia thing")
        .version("0.1")
        .arg(
            Arg::with_name("page")
                .short("p")
                .long("page")
                .value_name("PAGE")
                .help("Which page to scrape")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .help("Which wikipedia page to use")
                .takes_value(true),
        )
        .get_matches();

    let cmd = cli.value_of("page").unwrap_or("");
    let url = cli.value_of("url").unwrap_or(COMPOSERS_BY_NAME);
    let page = match cmd {
        "composers" => Some(COMPOSERS_BY_NAME),
        _ => Some(url),
    };
    if let Some(page_name) = page {
        println!("{}", rand_item_from_page(page_name)?);
    } else {
        println!("'{}' not understood", cmd);
    }
    Ok(())
}
