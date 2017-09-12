extern crate rand_wikipedia;

use std::fs::File;
use std::io::Read;

fn vec_str_to_vec_string(input: Vec<&str>) -> Vec<String> {
    input.iter().map(|x| x.to_string()).collect()
}

#[test]
fn test_parse_list_of_names() {
    use rand_wikipedia::Link;

    let mut f = File::open("tests/test-data.html").unwrap();
    let mut test_page = String::new();
    f.read_to_string(&mut test_page).unwrap();

    assert_eq!(
        rand_wikipedia::parse_page(test_page),
        vec![
            Link::new("Sara", "sara"),
            Link::new("Bob", "bob"),
            Link::new("Alice", "alice"),
        ]
    );
}

#[test]
fn test_parse_cached_page() {
    let mut f = File::open("tests/List_of_composers_by_name.html").unwrap();
    let mut test_page = String::new();
    f.read_to_string(&mut test_page).unwrap();

    let result = rand_wikipedia::parse_page(test_page);
    assert!(result.contains(&"Peter Ablinger".to_string()));
}

#[test]
fn test_exclude_a_list_from_names() {
    let names: Vec<String> = vec_str_to_vec_string(vec!["Sara", "Bob"]);
    let excluding = vec_str_to_vec_string(vec!["Bob"]);
    assert_eq!(
        vec!["Sara"],
        rand_wikipedia::names_excluding(&names, &excluding)
    );
}

#[test]
fn test_get_a_random_name() {
    let names = vec_str_to_vec_string(vec!["Sara", "Bob", "Alice"]);
    let we_got = rand_wikipedia::rand_name(&names);
    assert!(names.contains(&we_got));
}
