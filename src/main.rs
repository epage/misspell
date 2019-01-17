use clap::{App, Arg};
use ignore::Walk;
use std::fs::{metadata, File};
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn render(path: &Path, line_num: usize, word: &str, correction: &str) {
    println!("{}:{}: {:?} -> {}",  path.display(), line_num, word, correction);
}

fn tokenize<'l>(line: &'l str) -> impl Iterator<Item=String> + 'l {
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
}

/// check all lines of a file for misspelled words
fn process_file(path: &Path, dictionary: &Corrections, min_token: u64) {
    let file = File::open(path).expect("file is accessible because of walking");
    BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .for_each(|(i, line)| { // for each line
            tokenize(&line)
            .filter(|word: &String| word.len() >= min_token as usize)
            .for_each(|word: String| { // for each word in the line
                if let Some(correction) = dictionary.correct(&word) {
                    render(path, i + 1, &word, correction);
                }
            });
        });
}

struct Corrections<'s> {
    dict: HashMap<&'s str, &'s str>,
}

impl<'s> Corrections<'s> {
    /// transform a csv file in the form `misspelled_word,correction` to a HashMap for fast lookup
    fn new(csv_data: &'s str) -> Self {
        let mut ret = HashMap::new();
        csv_data.lines()
            .for_each(|line| {
                let pair = line.split(',').collect::<Vec<_>>();
                let pair = (pair[0], pair[1]);
                ret.insert(pair.0, pair.1);
            });
        Corrections { dict: ret }
    }

    fn correct(&self, word: &str) -> Option<&str> {
        self.dict.get(word).map(|s| *s)
    }
}

fn main() {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
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
    let words_map = Corrections::new(words);
    let min_token_length = matches.value_of("min_token_length").unwrap_or("3");
    let min_token_length: u64 = min_token_length.parse().unwrap();

    matches.values_of("files").expect("parameter exists because defaulted").collect::<Vec<_>>()
        .iter()
        .filter(|p| {
            let attrs = metadata(p).expect("path is accessible because of walking");
            attrs.is_dir()
        })
        .for_each(|file| {
            Walk::new(file).for_each(|entry| match entry {
                Ok(entry) => process_file(entry.path(), &words_map, min_token_length),
                Err(err) => println!("ERROR: {}", err),
            });
        });
}
