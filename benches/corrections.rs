#![feature(test)]

extern crate test;

#[bench]
fn load_corrections(b: &mut test::Bencher) {
    b.iter(|| misspell::Corrections::new(misspell::CORPUS));
}

#[bench]
fn correction(b: &mut test::Bencher) {
    let corrections = misspell::Corrections::new(misspell::CORPUS);
    assert_eq!(corrections.correct("successs"), Some("successes"));
    b.iter(|| corrections.correct("successs"));
}

#[bench]
fn no_correction(b: &mut test::Bencher) {
    let corrections = misspell::Corrections::new(misspell::CORPUS);
    assert_eq!(corrections.correct("success"), None);
    b.iter(|| corrections.correct("success"));
}

