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

    let words = misspell::CORPUS;
    let words_map = misspell::Corrections::new(words);
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
                misspell::process_file(entry.path(), &words_map, min_token_length)
            })
    });
}
