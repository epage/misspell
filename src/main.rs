use clap::{App, Arg};
use ignore::Walk;
use std::fs::{File};
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
            // eg: `dictionary:` -> `dictionary`
            word.to_lowercase()
            .chars()
            .filter(|x|
                match x {
                    'a'...'z' => true,
                    _ => false,
            }).collect()
        })
}

fn process_line(line_num: usize, line: &str, min_token: u64, path: &Path, dico: &HashMap<&str, &str>) {
    tokenize(line)
        .filter(|word: &String| word.len() >= min_token as usize)
        .for_each(|word: String| { // for each word in the line
            if let Some(correction) = dico.get(word.as_str()) {
                render(path, line_num + 1, &word, correction);
            }
        });
}

/// check all lines of a file for misspelled words
fn process_file(path: &Path, dictionary: &HashMap<&str, &str>, min_token: u64) {
    let file = File::open(path).expect(format!("opening file: {}", path.display()).as_str());
    BufReader::new(file).lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .for_each(|(line_num, line)|
            process_line(line_num, line.as_str(), min_token, path, dictionary)
        );
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

    let words_map = parse_words(include_str!("../assets/words.csv"));
    let min_token_length = matches.value_of("min_token_length").unwrap_or("3");
    let min_token_length: u64 = min_token_length.parse().expect("length argument is not valid");

    matches.values_of("files").expect("error opening files").collect::<Vec<_>>()
        .iter()
        .for_each(|file| { // for each file provided as CLI argument
            Walk::new(file)
            .filter_map(|entry| entry.ok()) // skip erroneous entries
            .filter(|entry| { // do not process folders
                let attrs = entry.metadata()
                    .expect(format!("reading metadata: {}", entry.path().display()).as_str());
                attrs.is_dir() == false
            })
            .for_each(|entry| { // for each file found while walking
                process_file(entry.path(), &words_map, min_token_length)
            })
    });
}
