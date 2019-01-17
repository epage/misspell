#![feature(test)]

extern crate test;

mod data;

#[bench]
fn tokenize_empty(b: &mut test::Bencher) {
    b.iter(|| misspell::tokenize(data::EMPTY).collect::<Vec<_>>());
}

#[bench]
fn tokenize_no_tokens(b: &mut test::Bencher) {
    b.iter(|| misspell::tokenize(data::NO_TOKENS).collect::<Vec<_>>());
}

#[bench]
fn tokenize_single_token(b: &mut test::Bencher) {
    b.iter(|| misspell::tokenize(data::SINGLE_TOKEN).collect::<Vec<_>>());
}

#[bench]
fn tokenize_sherlock(b: &mut test::Bencher) {
    b.iter(|| misspell::tokenize(data::SHERLOCK).collect::<Vec<_>>());
}

#[bench]
fn tokenize_code(b: &mut test::Bencher) {
    b.iter(|| misspell::tokenize(data::CODE).collect::<Vec<_>>());
}

#[bench]
fn tokenize_corpus(b: &mut test::Bencher) {
    b.iter(|| misspell::tokenize(misspell::CORPUS).collect::<Vec<_>>());
}
