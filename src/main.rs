mod info;

use clap::{App, Arg};
use ignore::Walk;
use std::fs::{metadata, File};
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

/// check all lines of a file for misspelled words
fn process_file(path: &Path, dictionary: &HashMap<&str, &str>, min_token: u64) {
    let attrs = metadata(path).expect("reading file metadta");
    if attrs.is_dir() {
        return;
    }
    let file = File::open(path).expect("opening file");
    BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .for_each(|(i, line)| { // for each line
            line.split_whitespace()
            .map(|word| {
                // lowercase word then remove all non alphabetical characters
                // eg: `dictionary`
                word.to_lowercase()
                .chars()
                .filter(|x|
                    match x {
                        'a'...'z' => true,
                        _ => false,
                }).collect()
            })
            .filter(|word: &String| word.len() >= min_token as usize)
            .for_each(|word: String| { // for each word in the line
                if let Some(correction) = dictionary.get(word.as_str()) {
                    println!("{}:{}: {:?} -> {}",  path.display(), i + 1, word, correction);
                }
            });
        });
}

/// transform a csv file in the form `misspelled_word,correction` to a HashMap for fast lookup
fn parse_words(csv_data: &str) -> HashMap<&str, &str> {
    let mut ret = HashMap::new();
    csv_data.lines()
        .for_each(|line| {
            let pair = line.split(',').collect::<Vec<_>>();
            let pair = (pair[0], pair[1]);
            ret.insert(pair.0, pair.1);
        });
     ret
}

fn main() {
    let matches = App::new(info::NAME)
        .author(info::AUTHOR)
        .version(info::VERSION)
        .about(info::DESCRIPTION)
        .arg(Arg::with_name("files")
            .multiple(true)
            .default_value(".")
            .help("Input files"),
        )
        .arg(Arg::with_name("min_token_length")
            .short("n")
            .long("length")
            .takes_value(true)
            .help("Minimum matched token length"),
        )
        .get_matches();

    let words = include_str!("../assets/words.csv");
    let words_map = parse_words(words);
    let min_token_length = matches.value_of("min_token_length").unwrap_or("3");
    let min_token_length: u64 = min_token_length.parse().unwrap();

    matches.values_of("files").expect("error opening files").collect::<Vec<_>>()
        .iter()
        .for_each(|file| {
            Walk::new(file).for_each(|entry| match entry {
                Ok(entry) => process_file(entry.path(), &words_map, min_token_length),
                Err(err) => println!("ERROR: {}", err),
            });
        });
}
