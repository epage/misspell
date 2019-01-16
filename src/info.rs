use clap::crate_version;
use std::env::consts;

pub static NAME: &str = "misspell";
pub static VERSION: &str = crate_version!();
pub static AUTHOR: &str = "Sylvain Kerkour <sylvain@bloom.sh>";
pub static DESCRIPTION: &str = "Correct commonly misspelled English words in source files 📖";
#[allow(dead_code)]
pub static OS: &str = consts::OS;
#[allow(dead_code)]
pub static ARCH: &str = consts::ARCH;
