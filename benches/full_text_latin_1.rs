extern crate boyer_moore_magiclen;
#[macro_use]
extern crate criterion;
extern crate regex;
extern crate needle;

mod full_text_search;

use std::fs;

use criterion::Criterion;

use full_text_search::*;

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

fn short_bm(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("short_bm", move |b| {
        b.iter(|| {
            let result = bm_search(&text, PATTERN_SHORT);

            assert_eq!(PATTERN_SHORT_RESULT_COUNT, result.len());
        })
    });
}

fn short_horspool(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("short_horspool", move |b| {
        b.iter(|| {
            let result = horspool_search(&text, PATTERN_SHORT);

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
            let result = character_search_char(&text, &pattern);

            assert_eq!(PATTERN_SHORT_RESULT_COUNT, result.len());
        })
    });
}

criterion_group!(short, short_native, short_regex, short_bm, short_horspool, short_latin_1, short_character);


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

fn long_bm(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("long_bm", move |b| {
        b.iter(|| {
            let result = bm_search(&text, PATTERN_LONG);

            assert_eq!(PATTERN_LONG_RESULT_COUNT, result.len());
        })
    });
}

fn long_horspool(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("long_horspool", move |b| {
        b.iter(|| {
            let result = horspool_search(&text, PATTERN_LONG);

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
            let result = character_search_char(&text, &pattern);

            assert_eq!(PATTERN_LONG_RESULT_COUNT, result.len());
        })
    });
}

criterion_group!(long, long_native, long_regex, long_bm, long_horspool, long_latin_1, long_character);

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

fn not_exist_short_bm(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("not_exist_short_bm", move |b| {
        b.iter(|| {
            let result = bm_search(&text, NOT_EXIST_PATTERN_SHORT);

            assert_eq!(0, result.len());
        })
    });
}

fn not_exist_short_horspool(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("not_exist_short_horspool", move |b| {
        b.iter(|| {
            let result = horspool_search(&text, NOT_EXIST_PATTERN_SHORT);

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
            let result = character_search_char(&text, &pattern);

            assert_eq!(0, result.len());
        })
    });
}

criterion_group!(not_exist_short, not_exist_short_native, not_exist_short_regex, not_exist_short_bm, not_exist_short_horspool, not_exist_short_latin_1, not_exist_short_character);

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

fn not_exist_long_bm(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("not_exist_long_bm", move |b| {
        b.iter(|| {
            let result = bm_search(&text, NOT_EXIST_PATTERN_LONG);

            assert_eq!(0, result.len());
        })
    });
}

fn not_exist_long_horspool(c: &mut Criterion) {
    let text = fs::read_to_string(TXT_PATH).unwrap();

    c.bench_function("not_exist_long_horspool", move |b| {
        b.iter(|| {
            let result = horspool_search(&text, NOT_EXIST_PATTERN_LONG);

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
            let result = character_search_char(&text, &pattern);

            assert_eq!(0, result.len());
        })
    });
}

criterion_group!(not_exist_long, not_exist_long_native, not_exist_long_regex, not_exist_long_bm, not_exist_long_horspool, not_exist_long_latin_1, not_exist_long_character);

criterion_main!(short, long, not_exist_short, not_exist_long);