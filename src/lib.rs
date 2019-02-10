extern crate rand;
extern crate scraper;

use rand::seq::SliceRandom;
use scraper::{Html, Selector};
use std::collections::HashSet;

const LIST_ITEMS: &'static str =
    "#bodyContent > #mw-content-text > div ul > li > a";

pub fn parse_page(page_contents: String) -> Vec<String> {
    let document = Html::parse_document(&page_contents);
    let selector = Selector::parse(LIST_ITEMS).unwrap();
    document
        .select(&selector)
        .map(|element| element.text().collect::<Vec<&str>>().join(" "))
        .collect::<Vec<String>>()
}

pub fn names_excluding(
    names: &Vec<String>,
    excluding: &Vec<String>,
) -> Vec<String> {
    let a: HashSet<_> = names.iter().cloned().collect();
    let b: HashSet<_> = excluding.iter().cloned().collect();
    a.difference(&b).cloned().collect()
}

pub fn rand_name(names: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    names.choose(&mut rng).unwrap().clone()
}
