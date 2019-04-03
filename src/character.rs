use std::fmt::{self, Formatter, Debug};
use std::collections::HashMap;

use core::slice::Iter;
use std::ops::Deref;

// TODO Searchable

pub trait BMCharacterSearchable {
    #[inline]
    fn len(&self) -> usize;

    #[inline]
    fn value_at(&self, index: usize) -> char;

    #[inline]
    fn iter(&self) -> Iter<char>;
}

impl<'a> BMCharacterSearchable for [char] {
    #[inline]
    fn len(&self) -> usize {
        <[char]>::len(self)
    }

    #[inline]
    fn value_at(&self, index: usize) -> char {
        self[index]
    }

    #[inline]
    fn iter(&self) -> Iter<char> {
        <[char]>::iter(self)
    }
}

impl<'a> BMCharacterSearchable for Vec<char> {
    #[inline]
    fn len(&self) -> usize {
        Vec::len(&self)
    }

    #[inline]
    fn value_at(&self, index: usize) -> char {
        self[index]
    }

    #[inline]
    fn iter(&self) -> Iter<char> {
        self.as_slice().iter()
    }
}

impl<T: BMCharacterSearchable> BMCharacterSearchable for &T {
    #[inline]
    fn len(&self) -> usize {
        <BMCharacterSearchable>::len(*self)
    }

    #[inline]
    fn value_at(&self, index: usize) -> char {
        <BMCharacterSearchable>::value_at(*self, index)
    }

    #[inline]
    fn iter(&self) -> Iter<char> {
        <BMCharacterSearchable>::iter(*self)
    }
}

// TODO BasCharShiftMap

pub struct BMCharacterBadCharShiftMap {
    t: HashMap<char, usize>
}

impl Debug for BMCharacterBadCharShiftMap {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            f.write_fmt(format_args!("BMCharacterBadCharShiftMap {{\n    t: {:?}\n}}", self.t))
        } else {
            f.write_fmt(format_args!("BMCharacterBadCharShiftMap {{ t: {:?} }}", self.t))
        }
    }
}

impl Deref for BMCharacterBadCharShiftMap {
    type Target = HashMap<char, usize>;

    #[inline]
    fn deref(&self) -> &HashMap<char, usize> {
        &self.t
    }
}

pub struct BMCharacterBadCharShiftMapRev {
    t: HashMap<char, usize>
}

impl Debug for BMCharacterBadCharShiftMapRev {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            f.write_fmt(format_args!("BMCharacterBadCharShiftMapRev {{\n    t: {:?}\n}}", self.t))
        } else {
            f.write_fmt(format_args!("BMCharacterBadCharShiftMapRev {{ t: {:?} }}", self.t))
        }
    }
}

impl Deref for BMCharacterBadCharShiftMapRev {
    type Target = HashMap<char, usize>;

    #[inline]
    fn deref(&self) -> &HashMap<char, usize> {
        &self.t
    }
}

impl BMCharacterBadCharShiftMap {
    pub fn create_bad_char_shift_map<T: BMCharacterSearchable>(pattern: T) -> Option<BMCharacterBadCharShiftMap> {
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
            t: bad_char_shift_map
        })
    }
}

impl BMCharacterBadCharShiftMapRev {
    pub fn create_bad_char_shift_map<T: BMCharacterSearchable>(pattern: T) -> Option<BMCharacterBadCharShiftMapRev> {
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

// TODO BM

/// Using Boyer-Moore-MagicLen to search character sub-sequences in any character sequence.
#[derive(Debug)]
pub struct BMCharacter {
    bad_char_shift_map: BMCharacterBadCharShiftMap,
    bad_char_shift_map_rev: BMCharacterBadCharShiftMapRev,
    pattern: Vec<char>,
}

impl BMCharacter {
    /// Create a `BMByte` instance from a pattern (the search needle).
    ///
    /// ```
    /// extern crate boyer_moore_magiclen;
    ///
    /// use boyer_moore_magiclen::BMCharacter;
    ///
    /// let bmc = BMCharacter::from(vec!['o', 'o', 'c', 'o', 'o']).unwrap();
    /// ```
    pub fn from<T: BMCharacterSearchable>(pattern: T) -> Option<BMCharacter> {
        let bad_char_shift_map = BMCharacterBadCharShiftMap::create_bad_char_shift_map(&pattern)?;
        let bad_char_shift_map_rev = BMCharacterBadCharShiftMapRev::create_bad_char_shift_map(&pattern)?;

        Some(BMCharacter {
            bad_char_shift_map,
            bad_char_shift_map_rev,
            pattern: pattern.iter().map(|&c| c).collect(),
        })
    }
}

// TODO Find Full

impl BMCharacter {
    /// Find and return the positions of all matched sub-sequences in any text (the haystack).
    ///
    /// ```
    /// extern crate boyer_moore_magiclen;
    ///
    /// use boyer_moore_magiclen::BMCharacter;
    ///
    /// let bmc = BMCharacter::from(vec!['o', 'o', 'c', 'o', 'o']).unwrap();
    ///
    /// assert_eq!(vec![1, 4, 7], bmc.find_full_all_in(vec!['c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o']));
    /// ```
    pub fn find_full_all_in<T: BMCharacterSearchable>(&self, text: T) -> Vec<usize> {
        find_full(text, &self.pattern, &self.bad_char_shift_map, 0)
    }

    /// Find and return the positions of matched sub-sequences in any text (the haystack). If the `limit` is set to `0`, all sub-sequences will be found.
    ///
    /// ```
    /// extern crate boyer_moore_magiclen;
    ///
    /// use boyer_moore_magiclen::BMCharacter;
    ///
    /// let bmc = BMCharacter::from(vec!['o', 'o', 'c', 'o', 'o']).unwrap();
    ///
    /// assert_eq!(vec![1, 4], bmc.find_full_in(vec!['c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o'], 2));
    /// ```
    pub fn find_full_in<T: BMCharacterSearchable>(&self, text: T, limit: usize) -> Vec<usize> {
        find_full(text, &self.pattern, &self.bad_char_shift_map, limit)
    }
}

impl BMCharacter {
    /// Find and return the positions of all matched sub-sequences in any text (the haystack) from its tail to its head.
    ///
    /// ```
    /// extern crate boyer_moore_magiclen;
    ///
    /// use boyer_moore_magiclen::BMCharacter;
    ///
    /// let bmc = BMCharacter::from(vec!['o', 'o', 'c', 'o', 'o']).unwrap();
    ///
    /// assert_eq!(vec![7, 4, 1], bmc.rfind_full_all_in(vec!['c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o']));
    /// ```
    pub fn rfind_full_all_in<T: BMCharacterSearchable>(&self, text: T) -> Vec<usize> {
        rfind_full(text, &self.pattern, &self.bad_char_shift_map_rev, 0)
    }

    /// Find and return the positions of matched sub-sequences in any text (the haystack) from its tail to its head. If the `limit` is set to `0`, all sub-sequences will be found.
    ///
    /// ```
    /// extern crate boyer_moore_magiclen;
    ///
    /// use boyer_moore_magiclen::BMCharacter;
    ///
    /// let bmc = BMCharacter::from(vec!['o', 'o', 'c', 'o', 'o']).unwrap();
    ///
    /// assert_eq!(vec![7, 4], bmc.rfind_full_in(vec!['c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o'], 2));
    /// ```
    pub fn rfind_full_in<T: BMCharacterSearchable>(&self, text: T, limit: usize) -> Vec<usize> {
        rfind_full(text, &self.pattern, &self.bad_char_shift_map_rev, limit)
    }
}

pub fn find_full<TT: BMCharacterSearchable, TP: BMCharacterSearchable>(text: TT, pattern: TP, bad_char_shift_map: &BMCharacterBadCharShiftMap, limit: usize) -> Vec<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;
    let pattern_len_inc = pattern_len + 1;

    let last_pattern_char = pattern.value_at(pattern_len_dec);

    let mut shift = 0;

    let end_index = text_len - pattern_len;

    let mut result = vec![];

    'outer: loop {
        for (i, &pc) in pattern.iter().enumerate().rev() {
            if text.value_at(shift + i) != pc {
                let p = shift + pattern_len;
                if p == text_len {
                    break 'outer;
                }
                shift += bad_char_shift_map.get(&text.value_at(shift + pattern_len_dec)).map(|&c| c).unwrap_or(pattern_len).max(
                    {
                        let c = text.value_at(p);

                        if c == last_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc)
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

        shift += bad_char_shift_map.get(&text.value_at(shift + pattern_len_dec)).map(|&c| c).unwrap_or(pattern_len).max(
            {
                let c = text.value_at(shift + pattern_len);

                if c == last_pattern_char {
                    1
                } else {
                    bad_char_shift_map.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc)
                }
            }
        );
        if shift > end_index {
            break;
        }
    }

    result
}

pub fn rfind_full<TT: BMCharacterSearchable, TP: BMCharacterSearchable>(text: TT, pattern: TP, bad_char_shift_map: &BMCharacterBadCharShiftMapRev, limit: usize) -> Vec<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;
    let pattern_len_inc = pattern_len + 1;

    let first_pattern_char = pattern.value_at(0);

    let mut shift = text_len - 1;

    let start_index = pattern_len_dec;

    let mut result = vec![];

    'outer: loop {
        for (i, &pc) in pattern.iter().enumerate() {
            if text.value_at(shift - pattern_len_dec + i) != pc {
                if shift < pattern_len {
                    break 'outer;
                }
                shift -= bad_char_shift_map.get(&text.value_at(shift - pattern_len_dec)).map(|&c| c).unwrap_or(pattern_len).max(
                    {
                        let c = text.value_at(shift - pattern_len);

                        if c == first_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc)
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

        shift -= bad_char_shift_map.get(&text.value_at(shift - pattern_len_dec)).map(|&c| c).unwrap_or(pattern_len).max(
            {
                let c = text.value_at(shift - pattern_len);

                if c == first_pattern_char {
                    1
                } else {
                    let s = bad_char_shift_map.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc);

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

// TODO Find

impl BMCharacter {
    /// Find and return the positions of all matched sub-sequences in any text (the haystack) but not including the overlap.
    ///
    /// ```
    /// extern crate boyer_moore_magiclen;
    ///
    /// use boyer_moore_magiclen::BMCharacter;
    ///
    /// let bmc = BMCharacter::from(vec!['o', 'o', 'c', 'o', 'o']).unwrap();
    ///
    /// assert_eq!(vec![1, 7], bmc.find_all_in(vec!['c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o']));
    /// ```
    pub fn find_all_in<T: BMCharacterSearchable>(&self, text: T) -> Vec<usize> {
        find(text, &self.pattern, &self.bad_char_shift_map, 0)
    }

    /// Find and return the position of the first matched sub-sequence in any text (the haystack).
    ///
    /// ```
    /// extern crate boyer_moore_magiclen;
    ///
    /// use boyer_moore_magiclen::BMCharacter;
    ///
    /// let bmc = BMCharacter::from(vec!['o', 'o', 'c', 'o', 'o']).unwrap();
    ///
    /// assert_eq!(Some(1), bmc.find_first_in(vec!['c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o']));
    /// ```
    pub fn find_first_in<T: BMCharacterSearchable>(&self, text: T) -> Option<usize> {
        find(text, &self.pattern, &self.bad_char_shift_map, 1).get(0).map(|&p| p)
    }

    /// Find and return the positions of matched sub-sequences in any text (the haystack) but not including the overlap. If the `limit` is set to `0`, all sub-sequences will be found.
    ///
    /// ```
    /// extern crate boyer_moore_magiclen;
    ///
    /// use boyer_moore_magiclen::BMCharacter;
    ///
    /// let bmc = BMCharacter::from(vec!['o', 'o', 'c', 'o', 'o']).unwrap();
    ///
    /// assert_eq!(vec![1], bmc.find_in(vec!['c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o'], 1));
    /// ```
    pub fn find_in<T: BMCharacterSearchable>(&self, text: T, limit: usize) -> Vec<usize> {
        find(text, &self.pattern, &self.bad_char_shift_map, limit)
    }
}

impl BMCharacter {
    /// Find and return the positions of all matched sub-sequences in any text (the haystack) but not including the overlap from its tail to its head.
    ///
    /// ```
    /// extern crate boyer_moore_magiclen;
    ///
    /// use boyer_moore_magiclen::BMCharacter;
    ///
    /// let bmc = BMCharacter::from(vec!['o', 'o', 'c', 'o', 'o']).unwrap();
    ///
    /// assert_eq!(vec![7, 1], bmc.rfind_all_in(vec!['c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o']));
    /// ```
    pub fn rfind_all_in<T: BMCharacterSearchable>(&self, text: T) -> Vec<usize> {
        rfind(text, &self.pattern, &self.bad_char_shift_map_rev, 0)
    }

    /// Find and return the position of the first matched sub-sequence in any text (the haystack) from its tail to its head.
    ///
    /// ```
    /// extern crate boyer_moore_magiclen;
    ///
    /// use boyer_moore_magiclen::BMCharacter;
    ///
    /// let bmc = BMCharacter::from(vec!['o', 'o', 'c', 'o', 'o']).unwrap();
    ///
    /// assert_eq!(Some(7), bmc.rfind_first_in(vec!['c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o']));
    /// ```
    pub fn rfind_first_in<T: BMCharacterSearchable>(&self, text: T) -> Option<usize> {
        rfind(text, &self.pattern, &self.bad_char_shift_map_rev, 1).get(0).map(|&p| p)
    }

    /// Find and return the positions of matched sub-sequences in any text (the haystack) but not including the overlap from its tail to its head. If the `limit` is set to `0`, all sub-sequences will be found.
    ///
    /// ```
    /// extern crate boyer_moore_magiclen;
    ///
    /// use boyer_moore_magiclen::BMCharacter;
    ///
    /// let bmc = BMCharacter::from(vec!['o', 'o', 'c', 'o', 'o']).unwrap();
    ///
    /// assert_eq!(vec![7], bmc.rfind_in(vec!['c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o', 'c', 'o', 'o'], 1));
    /// ```
    pub fn rfind_in<T: BMCharacterSearchable>(&self, text: T, limit: usize) -> Vec<usize> {
        rfind(text, &self.pattern, &self.bad_char_shift_map_rev, limit)
    }
}

pub fn find<TT: BMCharacterSearchable, TP: BMCharacterSearchable>(text: TT, pattern: TP, bad_char_shift_map: &BMCharacterBadCharShiftMap, limit: usize) -> Vec<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;
    let pattern_len_inc = pattern_len + 1;

    let last_pattern_char = pattern.value_at(pattern_len_dec);

    let mut shift = 0;

    let end_index = text_len - pattern_len;

    let mut result = vec![];

    'outer: loop {
        for (i, &pc) in pattern.iter().enumerate().rev() {
            if text.value_at(shift + i) != pc {
                let p = shift + pattern_len;
                if p == text_len {
                    break 'outer;
                }
                shift += bad_char_shift_map.get(&text.value_at(shift + pattern_len_dec)).map(|&c| c).unwrap_or(pattern_len).max(
                    {
                        let c = text.value_at(p);

                        if c == last_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc)
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

pub fn rfind<TT: BMCharacterSearchable, TP: BMCharacterSearchable>(text: TT, pattern: TP, bad_char_shift_map: &BMCharacterBadCharShiftMapRev, limit: usize) -> Vec<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;
    let pattern_len_inc = pattern_len + 1;

    let first_pattern_char = pattern.value_at(0);

    let mut shift = text_len - 1;

    let start_index = pattern_len_dec;

    let mut result = vec![];

    'outer: loop {
        for (i, &pc) in pattern.iter().enumerate() {
            if text.value_at(shift - pattern_len_dec + i) != pc {
                if shift < pattern_len {
                    break 'outer;
                }
                shift -= bad_char_shift_map.get(&text.value_at(shift - pattern_len_dec)).map(|&c| c).unwrap_or(pattern_len).max(
                    {
                        let c = text.value_at(shift - pattern_len);

                        if c == first_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.get(&c).map(|&c| c + 1).unwrap_or(pattern_len_inc)
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