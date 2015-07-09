use std::env;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::string::String;

fn main() {
    let path_arg = env::args().nth(1).unwrap();
    let path = Path::new(&path_arg);
    let file = File::open(&path).unwrap();
    let buffer = BufReader::new(file);
    buffer
        .lines()
        .map(|line| line.unwrap())
        .map(|string| format_string(string))
        .map(|string| println!("{}", string))
        .collect::<Vec<()>>();
}

fn format_string(string: String) -> String {
    let palindrome = is_palindrome(&string);
    let status = get_status(palindrome);
    let stripped_string = strip_spaces(&string);
    let sorted_string = sort_string(&stripped_string);
    let reversed_sorted = reverse(&sorted_string);
    return format!("{} | {}", status, reversed_sorted);
}

fn sort_string(string: &String) -> String {
    let mut characters: Vec<char> = string
        .chars()
        .collect();
    characters.sort();
    return characters
        .into_iter()
        .collect();
}

fn strip_spaces(string: &String) -> String {
    return string
        .chars()
        .filter(|&c| c != ' ')
        .collect::<String>();
}

fn get_status<'a>(boolean: bool) -> &'a str {
    let mut responses = HashMap::new();
    responses.insert(true, "YES");
    responses.insert(false, "NO");
    return responses.get(&boolean).unwrap()
}

fn is_palindrome(string: &String) -> bool {
    let lower_string = lowercase(&string);
    let reversed = reverse(&lower_string);
    return &lower_string == &reversed;
}

fn lowercase(string: &String) -> String {
    return string
        .chars()
        .map(lower_case_char)
        .collect::<String>();
}

fn lower_case_char(c: char) -> char {
    return c
        .to_lowercase()
        .next()
        .unwrap()
}

fn reverse(string: &String) -> String {
    return string
        .chars()
        .rev()
        .collect();
}
