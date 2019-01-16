use ignore::Walk;
use clap::{App};
use std::path::Path;
use std::fs::{metadata, File};
use std::io::{BufReader, Read};


fn process_file(path: &Path) {
    let attrs =  metadata(path).unwrap();
    if attrs.is_dir() {
        return;
    }
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    println!("{}:", path.display());
    println!("{}", contents);
}

fn main() {
    let matches = App::new("misspell")
        .arg(clap::Arg::with_name("files")
            .multiple(true)
            .help("Input files")
        )
        .get_matches();

    matches.values_of("files").unwrap().collect::<Vec<_>>().iter().for_each(|file| {
        Walk::new(file).for_each(|entry| {
        match entry {
            Ok(entry) => process_file(entry.path()),
            Err(err) => println!("ERROR: {}", err),
        }
    });
    });
}
