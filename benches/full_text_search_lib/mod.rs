use boyer_moore_magiclen::*;
use regex::Regex;

pub fn naive_search<S: AsRef<str>, P: AsRef<str>>(text: S, pattern: P) -> Vec<usize> {
    let text = text.as_ref();
    let pattern = pattern.as_ref();

    let length = text.len();

    let mut result = Vec::new();

    let mut offset = 0;

    let pattern_first_char_width =
        unsafe { utf8_width::get_width_assume_valid(pattern.as_bytes()[0]) };

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

    let regex = Regex::new(regex::escape(pattern).as_str()).unwrap();

    let length = text.len();

    let mut result = Vec::new();

    let mut offset = 0;

    let pattern_first_char_width =
        unsafe { utf8_width::get_width_assume_valid(pattern.as_bytes()[0]) };

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

pub fn bmb_search<TT: AsRef<[u8]>, TP: AsRef<[u8]>>(text: TT, pattern: TP) -> Vec<usize> {
    let bad_char_shift_map = BMByteBadCharShiftMap::create_bad_char_shift_map(&pattern).unwrap();

    boyer_moore_magiclen::byte::find_full(text.as_ref(), pattern.as_ref(), &bad_char_shift_map, 0)
}

#[cfg(feature = "character")]
pub fn character_search_char<TT: AsRef<[char]>, TP: AsRef<[char]>>(
    text: TT,
    pattern: TP,
) -> Vec<usize> {
    let bad_char_shift_map =
        BMCharacterBadCharShiftMap::create_bad_char_shift_map(&pattern).unwrap();

    character::find_full(text.as_ref(), pattern.as_ref(), &bad_char_shift_map, 0)
}
