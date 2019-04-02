extern crate boyer_moore_magiclen;
#[macro_use]
extern crate criterion;
extern crate regex;

mod search;

use std::fs;

use criterion::Criterion;

use search::*;

#[cfg(windows)]
const TXT_PATH: &'static str = r"benches\data\vgilante.txt";

#[cfg(not(windows))]
const TXT_PATH: &'static str = r"benches/data/vgilante.txt";

const PATTERN_SHORT: &'static str = "the";
const PATTERN_SHORT_RESULT_COUNT: usize = 5034;

const PATTERN_LONG: &'static str = "themselves";
const PATTERN_LONG_RESULT_COUNT: usize = 15;

const NOT_EXIST_PATTERN_SHORT: &'static str = "xyz";
const NOT_EXIST_PATTERN_LONG: &'static str = "xyzabcdefg";

fn short_native(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("short_native", move |b| {
        b.iter(|| {
            let result = native_search(&text, PATTERN_SHORT);

            assert_eq!(PATTERN_SHORT_RESULT_COUNT, result.len());
        })
    });
}

fn short_regex(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("short_regex", move |b| {
        b.iter(|| {
            let result = regex_search(&text, PATTERN_SHORT);

            assert_eq!(PATTERN_SHORT_RESULT_COUNT, result.len());
        })
    });
}

fn short_latin_1(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("short_latin_1", move |b| {
        b.iter(|| {
            let result = latin_1_search(&text, PATTERN_SHORT);

            assert_eq!(PATTERN_SHORT_RESULT_COUNT, result.len());
        })
    });
}

fn short_character(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = PATTERN_SHORT.chars().collect();

    c.bench_function("short_character", move |b| {
        b.iter(|| {
            let result = character_search(&text, &pattern);

            assert_eq!(PATTERN_SHORT_RESULT_COUNT, result.len());
        })
    });
}

criterion_group!(short, short_native, short_regex, short_latin_1, short_character);


fn long_native(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("long_native", move |b| {
        b.iter(|| {
            let result = native_search(&text, PATTERN_LONG);

            assert_eq!(PATTERN_LONG_RESULT_COUNT, result.len());
        })
    });
}

fn long_regex(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("long_regex", move |b| {
        b.iter(|| {
            let result = regex_search(&text, PATTERN_LONG);

            assert_eq!(PATTERN_LONG_RESULT_COUNT, result.len());
        })
    });
}

fn long_latin_1(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("long_latin_1", move |b| {
        b.iter(|| {
            let result = latin_1_search(&text, PATTERN_LONG);

            assert_eq!(PATTERN_LONG_RESULT_COUNT, result.len());
        })
    });
}

fn long_character(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = PATTERN_LONG.chars().collect();

    c.bench_function("long_character", move |b| {
        b.iter(|| {
            let result = character_search(&text, &pattern);

            assert_eq!(PATTERN_LONG_RESULT_COUNT, result.len());
        })
    });
}

criterion_group!(long, long_native, long_regex, long_latin_1, long_character);

fn not_exist_short_native(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("not_exist_short_native", move |b| {
        b.iter(|| {
            let result = native_search(&text, NOT_EXIST_PATTERN_SHORT);

            assert_eq!(0, result.len());
        })
    });
}

fn not_exist_short_regex(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("not_exist_short_regex", move |b| {
        b.iter(|| {
            let result = regex_search(&text, NOT_EXIST_PATTERN_SHORT);

            assert_eq!(0, result.len());
        })
    });
}

fn not_exist_short_latin_1(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("not_exist_short_latin_1", move |b| {
        b.iter(|| {
            let result = latin_1_search(&text, NOT_EXIST_PATTERN_SHORT);

            assert_eq!(0, result.len());
        })
    });
}

fn not_exist_short_character(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = NOT_EXIST_PATTERN_SHORT.chars().collect();

    c.bench_function("not_exist_short_character", move |b| {
        b.iter(|| {
            let result = character_search(&text, &pattern);

            assert_eq!(0, result.len());
        })
    });
}

criterion_group!(not_exist_short, not_exist_short_native, not_exist_short_regex, not_exist_short_latin_1, not_exist_short_character);

fn not_exist_long_native(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("not_exist_long_native", move |b| {
        b.iter(|| {
            let result = native_search(&text, NOT_EXIST_PATTERN_LONG);

            assert_eq!(0, result.len());
        })
    });
}

fn not_exist_long_regex(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("not_exist_long_regex", move |b| {
        b.iter(|| {
            let result = regex_search(&text, NOT_EXIST_PATTERN_LONG);

            assert_eq!(0, result.len());
        })
    });
}

fn not_exist_long_latin_1(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("not_exist_long_latin_1", move |b| {
        b.iter(|| {
            let result = latin_1_search(&text, NOT_EXIST_PATTERN_LONG);

            assert_eq!(0, result.len());
        })
    });
}

fn not_exist_long_character(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = NOT_EXIST_PATTERN_LONG.chars().collect();

    c.bench_function("not_exist_long_character", move |b| {
        b.iter(|| {
            let result = character_search(&text, &pattern);

            assert_eq!(0, result.len());
        })
    });
}

criterion_group!(not_exist_long, not_exist_long_native, not_exist_long_regex, not_exist_long_latin_1, not_exist_long_character);

criterion_main!(short, long, not_exist_short, not_exist_long);