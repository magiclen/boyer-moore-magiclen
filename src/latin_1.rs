use std::fmt::{self, Formatter, Debug};

use core::slice::Iter;
use std::ops::Deref;

// TODO Searchable

pub trait BMLatin1Searchable {
    #[inline]
    fn len(&self) -> usize;

    #[inline]
    fn value_at(&self, index: usize) -> u8;

    #[inline]
    fn iter(&self) -> Iter<u8>;
}

impl<'a> BMLatin1Searchable for String {
    #[inline]
    fn len(&self) -> usize {
        String::len(&self)
    }

    #[inline]
    fn value_at(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }

    #[inline]
    fn iter(&self) -> Iter<u8> {
        self.as_bytes().iter()
    }
}

impl<'a> BMLatin1Searchable for &str {
    #[inline]
    fn len(&self) -> usize {
        str::len(&self)
    }

    #[inline]
    fn value_at(&self, index: usize) -> u8 {
        unsafe {
            (*(*self as *const str as *const [u8]))[index]
        }
    }

    #[inline]
    fn iter(&self) -> Iter<u8> {
        self.as_bytes().iter()
    }
}

impl<'a> BMLatin1Searchable for [u8] {
    #[inline]
    fn len(&self) -> usize {
        <[u8]>::len(self)
    }

    #[inline]
    fn value_at(&self, index: usize) -> u8 {
        self[index]
    }

    #[inline]
    fn iter(&self) -> Iter<u8> {
        <[u8]>::iter(self)
    }
}

impl<'a> BMLatin1Searchable for Vec<u8> {
    #[inline]
    fn len(&self) -> usize {
        Vec::len(&self)
    }

    #[inline]
    fn value_at(&self, index: usize) -> u8 {
        self[index]
    }

    #[inline]
    fn iter(&self) -> Iter<u8> {
        self.as_slice().iter()
    }
}

impl<T: BMLatin1Searchable> BMLatin1Searchable for &T {
    #[inline]
    fn len(&self) -> usize {
        <BMLatin1Searchable>::len(*self)
    }

    #[inline]
    fn value_at(&self, index: usize) -> u8 {
        <BMLatin1Searchable>::value_at(*self, index)
    }

    #[inline]
    fn iter(&self) -> Iter<u8> {
        <BMLatin1Searchable>::iter(*self)
    }
}

// TODO BasCharShiftMap

pub struct BMLatin1BadCharShiftMap {
    t: [usize; 256]
}

impl Debug for BMLatin1BadCharShiftMap {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            f.write_fmt(format_args!("BMLatin1BadCharShiftMap {{\n    t: {:?}\n}}", self.t.as_ref()))
        } else {
            f.write_fmt(format_args!("BMLatin1BadCharShiftMap {{ t: {:?} }}", self.t.as_ref()))
        }
    }
}

impl Deref for BMLatin1BadCharShiftMap {
    type Target = [usize];

    #[inline]
    fn deref(&self) -> &[usize] {
        self.t.as_ref()
    }
}

pub struct BMLatin1BadCharShiftMapRev {
    t: [usize; 256]
}

impl Debug for BMLatin1BadCharShiftMapRev {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            f.write_fmt(format_args!("BMLatin1BadCharShiftMapRev {{\n    t: {:?}\n}}", self.t.as_ref()))
        } else {
            f.write_fmt(format_args!("BMLatin1BadCharShiftMapRev {{ t: {:?} }}", self.t.as_ref()))
        }
    }
}

impl Deref for BMLatin1BadCharShiftMapRev {
    type Target = [usize];

    #[inline]
    fn deref(&self) -> &[usize] {
        self.t.as_ref()
    }
}

impl BMLatin1BadCharShiftMap {
    pub fn create_bad_char_shift_map<T: BMLatin1Searchable>(pattern: T) -> Option<BMLatin1BadCharShiftMap> {
        let pattern_len = pattern.len();

        if pattern_len == 0 {
            return None;
        }

        let pattern_len_dec = pattern_len - 1;

        let mut bad_char_shift_map = [pattern_len; 256];

        for (i, c) in pattern.iter().take(pattern_len_dec).map(|&c| c as usize).enumerate() {
            bad_char_shift_map[c] = pattern_len_dec - i;
        }

        Some(BMLatin1BadCharShiftMap {
            t: bad_char_shift_map
        })
    }
}

impl BMLatin1BadCharShiftMapRev {
    pub fn create_bad_char_shift_map<T: BMLatin1Searchable>(pattern: T) -> Option<BMLatin1BadCharShiftMapRev> {
        let pattern_len = pattern.len();

        if pattern_len == 0 {
            return None;
        }

        let pattern_len_dec = pattern_len - 1;

        let mut bad_char_shift_map = [pattern_len; 256];

        for (i, c) in pattern.iter().enumerate().rev().take(pattern_len_dec).map(|(i, &c)| (i, c as usize)) {
            bad_char_shift_map[c] = i;
        }

        Some(BMLatin1BadCharShiftMapRev {
            t: bad_char_shift_map
        })
    }
}

// TODO BM

#[derive(Debug)]
pub struct BMLatin1 {
    bad_char_shift_map: BMLatin1BadCharShiftMap,
    bad_char_shift_map_rev: BMLatin1BadCharShiftMapRev,
    pattern: Vec<u8>,
}

impl BMLatin1 {
    pub fn from<T: BMLatin1Searchable>(pattern: T) -> Option<BMLatin1> {
        let bad_char_shift_map = BMLatin1BadCharShiftMap::create_bad_char_shift_map(&pattern)?;
        let bad_char_shift_map_rev = BMLatin1BadCharShiftMapRev::create_bad_char_shift_map(&pattern)?;

        Some(BMLatin1 {
            bad_char_shift_map,
            bad_char_shift_map_rev,
            pattern: pattern.iter().map(|&b| b).collect(),
        })
    }
}

// TODO Find Full

impl BMLatin1 {
    pub fn find_full_all_in<T: BMLatin1Searchable>(&self, text: T) -> Vec<usize> {
        find_full(text, &self.pattern, &self.bad_char_shift_map, 0)
    }

    pub fn find_full_first_in<T: BMLatin1Searchable>(&self, text: T) -> Option<usize> {
        find_full(text, &self.pattern, &self.bad_char_shift_map, 1).get(0).map(|&p| p)
    }

    pub fn find_full_in<T: BMLatin1Searchable>(&self, text: T, limit: usize) -> Vec<usize> {
        find_full(text, &self.pattern, &self.bad_char_shift_map, limit)
    }
}

impl BMLatin1 {
    pub fn rfind_full_all_in<T: BMLatin1Searchable>(&self, text: T) -> Vec<usize> {
        rfind_full(text, &self.pattern, &self.bad_char_shift_map_rev, 0)
    }

    pub fn rfind_full_first_in<T: BMLatin1Searchable>(&self, text: T) -> Option<usize> {
        rfind_full(text, &self.pattern, &self.bad_char_shift_map_rev, 1).get(0).map(|&p| p)
    }

    pub fn rfind_full_in<T: BMLatin1Searchable>(&self, text: T, limit: usize) -> Vec<usize> {
        rfind_full(text, &self.pattern, &self.bad_char_shift_map_rev, limit)
    }
}

pub fn find_full<TT: BMLatin1Searchable, TP: BMLatin1Searchable>(text: TT, pattern: TP, bad_char_shift_map: &BMLatin1BadCharShiftMap, limit: usize) -> Vec<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;

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
                shift += bad_char_shift_map[text.value_at(shift + pattern_len_dec) as usize].max(
                    {
                        let c = text.value_at(p);

                        if c == last_pattern_char {
                            1
                        } else {
                            bad_char_shift_map[c as usize] + 1
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

        shift += bad_char_shift_map[text.value_at(shift + pattern_len_dec) as usize].max(
            {
                let c = text.value_at(shift + pattern_len);

                if c == last_pattern_char {
                    1
                } else {
                    bad_char_shift_map[c as usize] + 1
                }
            }
        );
        if shift > end_index {
            break;
        }
    }

    result
}

pub fn rfind_full<TT: BMLatin1Searchable, TP: BMLatin1Searchable>(text: TT, pattern: TP, bad_char_shift_map: &BMLatin1BadCharShiftMapRev, limit: usize) -> Vec<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;

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
                shift -= bad_char_shift_map[text.value_at(shift - pattern_len_dec) as usize].max(
                    {
                        let c = text.value_at(shift - pattern_len);

                        if c == first_pattern_char {
                            1
                        } else {
                            bad_char_shift_map[c as usize] + 1
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

        shift -= bad_char_shift_map[text.value_at(shift - pattern_len_dec) as usize].max(
            {
                let c = text.value_at(shift - pattern_len);

                if c == first_pattern_char {
                    1
                } else {
                    let s = bad_char_shift_map[c as usize] + 1;

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

impl BMLatin1 {
    pub fn find_all_in<T: BMLatin1Searchable>(&self, text: T) -> Vec<usize> {
        find(text, &self.pattern, &self.bad_char_shift_map, 0)
    }

    pub fn find_first_in<T: BMLatin1Searchable>(&self, text: T) -> Option<usize> {
        find(text, &self.pattern, &self.bad_char_shift_map, 1).get(0).map(|&p| p)
    }

    pub fn find_in<T: BMLatin1Searchable>(&self, text: T, limit: usize) -> Vec<usize> {
        find(text, &self.pattern, &self.bad_char_shift_map, limit)
    }
}

impl BMLatin1 {
    pub fn rfind_all_in<T: BMLatin1Searchable>(&self, text: T) -> Vec<usize> {
        rfind(text, &self.pattern, &self.bad_char_shift_map_rev, 0)
    }

    pub fn rfind_first_in<T: BMLatin1Searchable>(&self, text: T) -> Option<usize> {
        rfind(text, &self.pattern, &self.bad_char_shift_map_rev, 1).get(0).map(|&p| p)
    }

    pub fn rfind_in<T: BMLatin1Searchable>(&self, text: T, limit: usize) -> Vec<usize> {
        rfind(text, &self.pattern, &self.bad_char_shift_map_rev, limit)
    }
}

pub fn find<TT: BMLatin1Searchable, TP: BMLatin1Searchable>(text: TT, pattern: TP, bad_char_shift_map: &BMLatin1BadCharShiftMap, limit: usize) -> Vec<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;

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
                shift += bad_char_shift_map[text.value_at(shift + pattern_len_dec) as usize].max(
                    {
                        let c = text.value_at(p);

                        if c == last_pattern_char {
                            1
                        } else {
                            bad_char_shift_map[c as usize] + 1
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

pub fn rfind<TT: BMLatin1Searchable, TP: BMLatin1Searchable>(text: TT, pattern: TP, bad_char_shift_map: &BMLatin1BadCharShiftMapRev, limit: usize) -> Vec<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;

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
                shift -= bad_char_shift_map[text.value_at(shift - pattern_len_dec) as usize].max(
                    {
                        let c = text.value_at(shift - pattern_len);

                        if c == first_pattern_char {
                            1
                        } else {
                            bad_char_shift_map[c as usize] + 1
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