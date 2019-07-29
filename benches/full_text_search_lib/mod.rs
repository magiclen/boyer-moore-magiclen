extern crate regex;
extern crate boyer_moore_magiclen;

mod utf8_width;

use self::regex::Regex;
use self::boyer_moore_magiclen::*;

pub fn naive_search<S: AsRef<str>, P: AsRef<str>>(text: S, pattern: P) -> Vec<usize> {
    let text = text.as_ref();
    let pattern = pattern.as_ref();

    let length = text.len();

    let mut result = Vec::new();

    let mut offset = 0;

    let pattern_first_char_width = utf8_width::utf8_char_width(pattern.as_bytes()[0]);

    while offset < length {
        if let Some(index) = text[offset..].find(pattern) {
            let index = index + offset;

            offset = index + pattern_first_char_width;

            result.push(index);
        } else {
            break;
        }
    }

    result
}

pub fn regex_search<S: AsRef<str>, P: AsRef<str>>(text: S, pattern: P) -> Vec<usize> {
    let text = text.as_ref();
    let pattern = pattern.as_ref();

    let regex = Regex::new(&format!("{}", regex::escape(pattern))).unwrap();

    let length = text.len();

    let mut result = Vec::new();

    let mut offset = 0;

    let pattern_first_char_width = utf8_width::utf8_char_width(pattern.as_bytes()[0]);

    while offset < length {
        if let Some(m) = regex.find(&text[offset..]) {
            let index = m.start() + offset;

            offset = index + pattern_first_char_width;

            result.push(index);
        } else {
            break;
        }
    }

    result
}

pub fn bmb_search<TT: BMByteSearchable, TP: BMByteSearchable>(text: TT, pattern: TP) -> Vec<usize> {
    let bad_char_shift_map = BMByteBadCharShiftMap::create_bad_char_shift_map(&pattern).unwrap();

    boyer_moore_magiclen::byte::find_full(text, pattern, &bad_char_shift_map, 0)
}

#[cfg(feature = "character")]
pub fn character_search_char<TT: BMCharacterSearchable, TP: BMCharacterSearchable>(text: TT, pattern: TP) -> Vec<usize> {
    let bad_char_shift_map = BMCharacterBadCharShiftMap::create_bad_char_shift_map(&pattern).unwrap();

    boyer_moore_magiclen::character::find_full(text, pattern, &bad_char_shift_map, 0)
}