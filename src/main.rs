use std::fs::metadata;

use clap::{App, Arg};
use ignore::Walk;

use misspell;

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
    let words_map = misspell::Corrections::new(words);
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
                Ok(entry) => misspell::process_file(entry.path(), &words_map, min_token_length),
                Err(err) => println!("ERROR: {}", err),
            });
        });
}
