use std::fmt::{self, Formatter, Debug};
use std::collections::HashMap;

pub struct BMCharacterBadCharShiftMap {
    t: HashMap<char, usize>
}

impl Debug for BMCharacterBadCharShiftMap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            f.write_fmt(format_args!("BMCharacterBadCharShiftMap {{\n    t: {:?}\n}}", self.t))
        } else {
            f.write_fmt(format_args!("BMCharacterBadCharShiftMap {{ t: {:?} }}", self.t))
        }
    }
}

pub struct BMCharacterBadCharShiftMapRev {
    t: HashMap<char, usize>
}

impl Debug for BMCharacterBadCharShiftMapRev {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            f.write_fmt(format_args!("BMCharacterBadCharShiftMapRev {{\n    t: {:?}\n}}", self.t))
        } else {
            f.write_fmt(format_args!("BMCharacterBadCharShiftMapRev {{ t: {:?} }}", self.t))
        }
    }
}

impl BMCharacterBadCharShiftMap {
    pub fn create_bad_char_shift_map_from_char_slice(pattern: &[char]) -> Option<BMCharacterBadCharShiftMap> {
        let pattern = pattern.as_ref();

        let pattern_len = pattern.len();

        if pattern_len == 0 {
            return None;
        }

        let pattern_len_dec = pattern_len - 1;

        let mut bad_char_shift_map: HashMap<char, usize> = HashMap::with_capacity(pattern_len_dec);

        for (i, &c) in pattern.iter().take(pattern_len_dec).enumerate() {
            bad_char_shift_map.insert(c, pattern_len_dec - i);
        }

        Some(BMCharacterBadCharShiftMap {
            t: bad_char_shift_map,
        })
    }
}

impl BMCharacterBadCharShiftMapRev {
    pub fn create_bad_char_shift_map_from_char_slice(pattern: &[char]) -> Option<BMCharacterBadCharShiftMapRev> {
        let pattern = pattern.as_ref();

        let pattern_len = pattern.len();

        if pattern_len == 0 {
            return None;
        }

        let pattern_len_dec = pattern_len - 1;

        let mut bad_char_shift_map: HashMap<char, usize> = HashMap::with_capacity(pattern_len_dec);

        for (i, &c) in pattern.iter().enumerate().rev().take(pattern_len_dec) {
            bad_char_shift_map.insert(c, i);
        }

        Some(BMCharacterBadCharShiftMapRev {
            t: bad_char_shift_map
        })
    }
}

#[derive(Debug)]
pub struct BMCharacter {
    bad_char_shift_map: BMCharacterBadCharShiftMap,
    bad_char_shift_map_rev: BMCharacterBadCharShiftMapRev,
    pattern: Vec<char>,
}

impl BMCharacter {
    pub fn from_char_slice(pattern: &[char]) -> Option<BMCharacter> {
        let bad_char_shift_map = BMCharacterBadCharShiftMap::create_bad_char_shift_map_from_char_slice(pattern)?;
        let bad_char_shift_map_rev = BMCharacterBadCharShiftMapRev::create_bad_char_shift_map_from_char_slice(pattern)?;

        Some(BMCharacter {
            bad_char_shift_map,
            bad_char_shift_map_rev,
            pattern: pattern.iter().map(|&c| c).collect(),
        })
    }
}

// TODO Find Full

impl BMCharacter {
    pub fn find_full_all_in_char_slice(&self, text: &[char]) -> Vec<usize> {
        find_full_in_char_slice(text, &self.pattern, &self.bad_char_shift_map, 0)
    }

    pub fn find_full_first_in_char_slice(&self, text: &[char]) -> Option<usize> {
        find_full_in_char_slice(text, &self.pattern, &self.bad_char_shift_map, 1).get(0).map(|&p| p)
    }

    pub fn find_full_in_char_slice(&self, text: &[char], limit: usize) -> Vec<usize> {
        find_full_in_char_slice(text, &self.pattern, &self.bad_char_shift_map, limit)
    }
}

impl BMCharacter {
    pub fn rfind_full_all_in_char_slice(&self, text: &[char]) -> Vec<usize> {
        rfind_full_in_char_slice(text, &self.pattern, &self.bad_char_shift_map_rev, 0)
    }

    pub fn rfind_full_first_in_char_slice(&self, text: &[char]) -> Option<usize> {
        rfind_full_in_char_slice(text, &self.pattern, &self.bad_char_shift_map_rev, 1).get(0).map(|&p| p)
    }

    pub fn rfind_full_in_char_slice(&self, text: &[char], limit: usize) -> Vec<usize> {
        rfind_full_in_char_slice(text, &self.pattern, &self.bad_char_shift_map_rev, limit)
    }
}

pub fn find_full_in_char_slice(text: &[char], pattern: &[char], bad_char_shift_map: &BMCharacterBadCharShiftMap, limit: usize) -> Vec<usize> {
    let text = text.as_ref();
    let pattern = pattern.as_ref();

    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;
    let pattern_len_inc = pattern_len + 1;

    let last_pattern_char = pattern[pattern_len_dec];

    let mut shift = 0;

    let end_index = text_len - pattern_len;

    let mut result = vec![];

    'outer: loop {
        for (i, &pc) in pattern.iter().enumerate().rev() {
            if text[shift + i] != pc {
                let p = shift + pattern_len;
                if p == text_len {
                    break 'outer;
                }
                shift += bad_char_shift_map.t.get(&text[shift + pattern_len_dec]).map(|&c| c).unwrap_or(pattern_len).max(
                    {
                        let c = text[p];

                        if c == last_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.t.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc)
                        }
                    }
                );
                if shift > end_index {
                    break 'outer;
                }
                continue 'outer;
            }
        }
        result.push(shift);

        if shift == end_index {
            break;
        }

        if result.len() == limit {
            break;
        }

        shift += bad_char_shift_map.t.get(&text[shift + pattern_len_dec]).map(|&c| c).unwrap_or(pattern_len).max(
            {
                let c = text[shift + pattern_len];

                if c == last_pattern_char {
                    1
                } else {
                    bad_char_shift_map.t.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc)
                }
            }
        );
        if shift > end_index {
            break;
        }
    }

    result
}

pub fn rfind_full_in_char_slice(text: &[char], pattern: &[char], bad_char_shift_map: &BMCharacterBadCharShiftMapRev, limit: usize) -> Vec<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;
    let pattern_len_inc = pattern_len + 1;

    let first_pattern_char = pattern[0];

    let mut shift = text_len - 1;

    let start_index = pattern_len_dec;

    let mut result = vec![];

    'outer: loop {
        for (i, &pc) in pattern.iter().enumerate() {
            if text[shift - pattern_len_dec + i] != pc {
                if shift < pattern_len {
                    break 'outer;
                }
                shift -= bad_char_shift_map.t.get(&text[shift - pattern_len_dec]).map(|&c| c).unwrap_or(pattern_len).max(
                    {
                        let c = text[shift - pattern_len];

                        if c == first_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.t.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc)
                        }
                    }
                );
                if shift < start_index {
                    break 'outer;
                }
                continue 'outer;
            }
        }
        result.push(shift - pattern_len_dec);

        if shift == start_index {
            break;
        }

        if result.len() == limit {
            break;
        }

        shift -= bad_char_shift_map.t.get(&text[shift - pattern_len_dec]).map(|&c| c).unwrap_or(pattern_len).max(
            {
                let c = text[shift - pattern_len];

                if c == first_pattern_char {
                    1
                } else {
                    let s = bad_char_shift_map.t.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc);

                    if shift < s {
                        break;
                    }

                    s
                }
            }
        );
        if shift < start_index {
            break;
        }
    }

    result
}

// TODO Full

impl BMCharacter {
    pub fn find_all_in_char_slice(&self, text: &[char]) -> Vec<usize> {
        find_in_char_slice(text, &self.pattern, &self.bad_char_shift_map, 0)
    }

    pub fn find_first_in_char_slice(&self, text: &[char]) -> Option<usize> {
        find_in_char_slice(text, &self.pattern, &self.bad_char_shift_map, 1).get(0).map(|&p| p)
    }

    pub fn find_in_char_slice(&self, text: &[char], limit: usize) -> Vec<usize> {
        find_in_char_slice(text, &self.pattern, &self.bad_char_shift_map, limit)
    }
}

impl BMCharacter {
    pub fn rfind_all_in_char_slice(&self, text: &[char]) -> Vec<usize> {
        rfind_in_char_slice(text, &self.pattern, &self.bad_char_shift_map_rev, 0)
    }

    pub fn rfind_first_in_char_slice(&self, text: &[char]) -> Option<usize> {
        rfind_in_char_slice(text, &self.pattern, &self.bad_char_shift_map_rev, 1).get(0).map(|&p| p)
    }

    pub fn rfind_in_char_slice(&self, text: &[char], limit: usize) -> Vec<usize> {
        rfind_in_char_slice(text, &self.pattern, &self.bad_char_shift_map_rev, limit)
    }
}

pub fn find_in_char_slice(text: &[char], pattern: &[char], bad_char_shift_map: &BMCharacterBadCharShiftMap, limit: usize) -> Vec<usize> {
    let text = text.as_ref();
    let pattern = pattern.as_ref();

    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;
    let pattern_len_inc = pattern_len + 1;

    let last_pattern_char = pattern[pattern_len_dec];

    let mut shift = 0;

    let end_index = text_len - pattern_len;

    let mut result = vec![];

    'outer: loop {
        for (i, &pc) in pattern.iter().enumerate().rev() {
            if text[shift + i] != pc {
                let p = shift + pattern_len;
                if p == text_len {
                    break 'outer;
                }
                shift += bad_char_shift_map.t.get(&text[shift + pattern_len_dec]).map(|&c| c).unwrap_or(pattern_len).max(
                    {
                        let c = text[p];

                        if c == last_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.t.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc)
                        }
                    }
                );
                if shift > end_index {
                    break 'outer;
                }
                continue 'outer;
            }
        }
        result.push(shift);

        if shift == end_index {
            break;
        }

        if result.len() == limit {
            break;
        }

        shift += pattern_len;
        if shift > end_index {
            break;
        }
    }

    result
}

pub fn rfind_in_char_slice(text: &[char], pattern: &[char], bad_char_shift_map: &BMCharacterBadCharShiftMapRev, limit: usize) -> Vec<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;
    let pattern_len_inc = pattern_len + 1;

    let first_pattern_char = pattern[0];

    let mut shift = text_len - 1;

    let start_index = pattern_len_dec;

    let mut result = vec![];

    'outer: loop {
        for (i, &pc) in pattern.iter().enumerate() {
            if text[shift - pattern_len_dec + i] != pc {
                if shift < pattern_len {
                    break 'outer;
                }
                shift -= bad_char_shift_map.t.get(&text[shift - pattern_len_dec]).map(|&c| c).unwrap_or(pattern_len).max(
                    {
                        let c = text[shift - pattern_len];

                        if c == first_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.t.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc)
                        }
                    }
                );
                if shift < start_index {
                    break 'outer;
                }
                continue 'outer;
            }
        }
        result.push(shift - pattern_len_dec);

        if shift == start_index {
            break;
        }

        if result.len() == limit {
            break;
        }

        shift -= pattern_len;
        if shift < start_index {
            break;
        }
    }

    result
}