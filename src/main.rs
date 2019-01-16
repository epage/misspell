mod info;

use clap::App;
use ignore::Walk;
use std::fs::{metadata, File};
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

/// check all lines of a file for misspelled words
fn process_file(path: &Path, dictionary: &HashMap<&str, &str>) {
    let attrs = metadata(path).unwrap();
    if attrs.is_dir() {
        return;
    }
    let file = File::open(path).unwrap();
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
            .for_each(|word: String| { // for each word in the line
                if dictionary.contains_key(word.as_str()) {
                    println!("{}:{}: {:?} -> {}",  path.display(), i + 1, word,
                        dictionary.get(word.as_str()).unwrap()
                    );
                }
            });
        });
}

/// transform a csv file in the form `mispelled_word,correction` to a HashMap for fast lookup
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
        .about(info::DESCRPITION)
        .arg(
            clap::Arg::with_name("files")
                .multiple(true)
                .default_value(".")
                .help("Input files"),
        )
        .get_matches();

    let words = include_str!("../assets/words.csv");
    let words_map = parse_words(words);

    matches.values_of("files").unwrap().collect::<Vec<_>>()
        .iter()
        .for_each(|file| {
            Walk::new(file).for_each(|entry| match entry {
                Ok(entry) => process_file(entry.path(), &words_map),
                Err(err) => println!("ERROR: {}", err),
            });
        });
}
