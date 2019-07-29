extern crate boyer_moore_magiclen;
#[macro_use]
extern crate bencher;
extern crate regex;

mod full_text_search_lib;

use std::fs;

use bencher::Bencher;

use full_text_search_lib::*;

#[cfg(windows)]
const TXT_PATH: &'static str = r"benches\data\vgilante.txt";

#[cfg(not(windows))]
const TXT_PATH: &'static str = r"benches/data/vgilante.txt";

const PATTERN_SHORT: &'static str = "the";
const PATTERN_SHORT_RESULT_COUNT: usize = 5034;

const PATTERN_LONG: &'static str = "Half the screen showed a graphic representation of what the
scanners had picked up the other side showed an analysis of the
same data.  The graphics showed an irregular shaped lump fade
on, stay several frames, then fade out.  At the time the lump
reminded on screen the analysis showed size about a quarter that
of the ship they had seen and mass as undetermined.";
const PATTERN_LONG_RESULT_COUNT: usize = 1;

const NOT_EXIST_PATTERN_SHORT: &'static str = "xyz";
const NOT_EXIST_PATTERN_LONG: &'static str = "xyzabcdefghijklmnopqrstuvwzyz xyzabcdefghijklmnopqrstuvwzyz
xyzabcdefghijklmnopqrstuvwzyz xyzabcdefghijklmnopqrstuvwzyz
xyzabcdefghijklmnopqrstuvwzyz xyzabcdefghijklmnopqrstuvwzyz
xyzabcdefghijklmnopqrstuvwzyz xyzabcdefghijklmnopqrstuvwzyz
xyzabcdefghijklmnopqrstuvwzyz xyzabcdefghijklmnopqrstuvwzyz";

fn short_naive(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = naive_search(&text, PATTERN_SHORT);

        assert_eq!(PATTERN_SHORT_RESULT_COUNT, result.len());
    });
}

fn short_regex(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = regex_search(&text, PATTERN_SHORT);

        assert_eq!(PATTERN_SHORT_RESULT_COUNT, result.len());
    });
}

fn short_bmb(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = bmb_search(text.as_str(), PATTERN_SHORT);

        assert_eq!(PATTERN_SHORT_RESULT_COUNT, result.len());
    });
}

#[cfg(feature = "character")]
fn short_character(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = PATTERN_SHORT.chars().collect();

    bencher.iter(|| {
        let result = character_search_char(&text, &pattern);

        assert_eq!(PATTERN_SHORT_RESULT_COUNT, result.len());
    });
}

#[cfg(feature = "character")]
benchmark_group!(short, short_naive, short_regex, short_bmb, short_character);

#[cfg(not(feature = "character"))]
benchmark_group!(short, short_naive, short_regex, short_bmb);


fn long_naive(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = naive_search(&text, PATTERN_LONG);

        assert_eq!(PATTERN_LONG_RESULT_COUNT, result.len());
    });
}

fn long_regex(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = regex_search(&text, PATTERN_LONG);

        assert_eq!(PATTERN_LONG_RESULT_COUNT, result.len());
    });
}

fn long_bmb(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = bmb_search(text.as_str(), PATTERN_LONG);

        assert_eq!(PATTERN_LONG_RESULT_COUNT, result.len());
    });
}

#[cfg(feature = "character")]
fn long_character(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = PATTERN_LONG.chars().collect();

    bencher.iter(|| {
        let result = character_search_char(&text, &pattern);

        assert_eq!(PATTERN_LONG_RESULT_COUNT, result.len());
    });
}

#[cfg(feature = "character")]
benchmark_group!(long, long_naive, long_regex, long_bmb, long_character);

#[cfg(not(feature = "character"))]
benchmark_group!(long, long_naive, long_regex, long_bmb);

fn not_exist_short_naive(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = naive_search(&text, NOT_EXIST_PATTERN_SHORT);

        assert_eq!(0, result.len());
    });
}

fn not_exist_short_regex(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = regex_search(&text, NOT_EXIST_PATTERN_SHORT);

        assert_eq!(0, result.len());
    });
}

fn not_exist_short_bmb(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = bmb_search(text.as_str(), NOT_EXIST_PATTERN_SHORT);

        assert_eq!(0, result.len());
    });
}

#[cfg(feature = "character")]
fn not_exist_short_character(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = NOT_EXIST_PATTERN_SHORT.chars().collect();

    bencher.iter(|| {
        let result = character_search_char(&text, &pattern);

        assert_eq!(0, result.len());
    });
}

#[cfg(feature = "character")]
benchmark_group!(not_exist_short, not_exist_short_naive, not_exist_short_regex, not_exist_short_bm, not_exist_short_horspool, not_exist_short_bmb, not_exist_short_character);

#[cfg(not(feature = "character"))]
benchmark_group!(not_exist_short, not_exist_short_naive, not_exist_short_regex, not_exist_short_bmb);

fn not_exist_long_naive(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = naive_search(&text, NOT_EXIST_PATTERN_LONG);

        assert_eq!(0, result.len());
    });
}

fn not_exist_long_regex(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = regex_search(&text, NOT_EXIST_PATTERN_LONG);

        assert_eq!(0, result.len());
    });
}

fn not_exist_long_bmb(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    bencher.iter(|| {
        let result = bmb_search(text.as_str(), NOT_EXIST_PATTERN_LONG);

        assert_eq!(0, result.len());
    });
}

#[cfg(feature = "character")]
fn not_exist_long_character(bencher: &mut Bencher) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = NOT_EXIST_PATTERN_LONG.chars().collect();

    bencher.iter(|| {
        let result = character_search_char(&text, &pattern);

        assert_eq!(0, result.len());
    });
}

#[cfg(feature = "character")]
benchmark_group!(not_exist_long, not_exist_long_naive, not_exist_long_regex, not_exist_long_bmb, not_exist_long_character);

#[cfg(not(feature = "character"))]
benchmark_group!(not_exist_long, not_exist_long_naive, not_exist_long_regex, not_exist_long_bmb);

benchmark_main!(short, long, not_exist_short, not_exist_long);