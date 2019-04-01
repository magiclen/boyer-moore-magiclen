use std::fmt::{self, Formatter, Debug};

pub struct BMLatin1BadCharShiftMap {
    t: [usize; 256]
}

impl Debug for BMLatin1BadCharShiftMap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            f.write_fmt(format_args!("BMLatin1BadCharShiftMap {{\n    t: {:?}\n}}", self.t.as_ref()))
        } else {
            f.write_fmt(format_args!("BMLatin1BadCharShiftMap {{ t: {:?} }}", self.t.as_ref()))
        }
    }
}

pub struct BMLatin1BadCharShiftMapRev {
    t: [usize; 256]
}

impl Debug for BMLatin1BadCharShiftMapRev {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            f.write_fmt(format_args!("BMLatin1BadCharShiftMapRev {{\n    t: {:?}\n}}", self.t.as_ref()))
        } else {
            f.write_fmt(format_args!("BMLatin1BadCharShiftMapRev {{ t: {:?} }}", self.t.as_ref()))
        }
    }
}

impl BMLatin1BadCharShiftMap {
    pub fn create_bad_char_shift_map_from_u8_slice<B: ?Sized + AsRef<[u8]>>(pattern: &B) -> Option<BMLatin1BadCharShiftMap> {
        let pattern = pattern.as_ref();

        let pattern_len = pattern.len();

        if pattern_len == 0 {
            return None;
        }

        let pattern_len_dec = pattern_len - 1;

        let mut bad_char_shift_map = [pattern_len; 256];

        for (i, &c) in pattern.iter().take(pattern_len_dec).enumerate() {
            bad_char_shift_map[c as usize] = pattern_len_dec - i;
        }

        Some(BMLatin1BadCharShiftMap {
            t: bad_char_shift_map
        })
    }

    pub fn create_bad_char_shift_map_from_str<S: AsRef<str>>(pattern: S) -> Option<BMLatin1BadCharShiftMap> {
        let pattern = pattern.as_ref();

        let pattern_len = pattern.len();

        if pattern_len == 0 {
            return None;
        }

        let pattern_len_dec = pattern_len - 1;

        let mut bad_char_shift_map = [pattern_len; 256];

        for (i, c) in pattern.bytes().take(pattern_len_dec).enumerate() {
            bad_char_shift_map[c as usize] = pattern_len_dec - i;
        }

        Some(BMLatin1BadCharShiftMap {
            t: bad_char_shift_map
        })
    }

    pub fn create_bad_char_shift_map_from_char_slice(pattern: &[char]) -> Option<BMLatin1BadCharShiftMap> {
        let pattern = pattern.as_ref();

        let pattern_len = pattern.len();

        if pattern_len == 0 {
            return None;
        }

        let pattern_len_dec = pattern_len - 1;

        let mut bad_char_shift_map = [pattern_len; 256];

        for (i, &c) in pattern.iter().take(pattern_len_dec).enumerate() {
            bad_char_shift_map[c as usize] = pattern_len_dec - i;
        }

        Some(BMLatin1BadCharShiftMap {
            t: bad_char_shift_map,
        })
    }
}

impl BMLatin1BadCharShiftMapRev {
    pub fn create_bad_char_shift_map_from_u8_slice<B: ?Sized + AsRef<[u8]>>(pattern: &B) -> Option<BMLatin1BadCharShiftMapRev> {
        let pattern = pattern.as_ref();

        let pattern_len = pattern.len();

        if pattern_len == 0 {
            return None;
        }

        let pattern_len_dec = pattern_len - 1;

        let mut bad_char_shift_map = [pattern_len; 256];

        for (i, &c) in pattern.iter().enumerate().rev().take(pattern_len_dec) {
            bad_char_shift_map[c as usize] = i;
        }

        Some(BMLatin1BadCharShiftMapRev {
            t: bad_char_shift_map
        })
    }

    pub fn create_bad_char_shift_map_from_str<S: AsRef<str>>(pattern: S) -> Option<BMLatin1BadCharShiftMapRev> {
        let pattern = pattern.as_ref();

        let pattern_len = pattern.len();

        if pattern_len == 0 {
            return None;
        }

        let pattern_len_dec = pattern_len - 1;

        let mut bad_char_shift_map = [pattern_len; 256];

        for (i, c) in pattern.bytes().enumerate().rev().take(pattern_len_dec) {
            bad_char_shift_map[c as usize] = i;
        }

        Some(BMLatin1BadCharShiftMapRev {
            t: bad_char_shift_map
        })
    }

    pub fn create_bad_char_shift_map_from_char_slice(pattern: &[char]) -> Option<BMLatin1BadCharShiftMapRev> {
        let pattern = pattern.as_ref();

        let pattern_len = pattern.len();

        if pattern_len == 0 {
            return None;
        }

        let pattern_len_dec = pattern_len - 1;

        let mut bad_char_shift_map = [pattern_len; 256];

        for (i, &c) in pattern.iter().enumerate().rev().take(pattern_len_dec) {
            bad_char_shift_map[c as usize] = i;
        }

        Some(BMLatin1BadCharShiftMapRev {
            t: bad_char_shift_map
        })
    }
}

#[derive(Debug)]
pub struct BMLatin1 {
    bad_char_shift_map: BMLatin1BadCharShiftMap,
    bad_char_shift_map_rev: BMLatin1BadCharShiftMapRev,
    pattern: Vec<u8>,
}

impl BMLatin1 {
    pub fn from_u8_slice<B: ?Sized + AsRef<[u8]>>(pattern: &B) -> Option<BMLatin1> {
        let bad_char_shift_map = BMLatin1BadCharShiftMap::create_bad_char_shift_map_from_u8_slice(pattern)?;
        let bad_char_shift_map_rev = BMLatin1BadCharShiftMapRev::create_bad_char_shift_map_from_u8_slice(pattern)?;

        Some(BMLatin1 {
            bad_char_shift_map,
            bad_char_shift_map_rev,
            pattern: pattern.as_ref().iter().map(|&b| b).collect(),
        })
    }

    pub fn from_str<S: AsRef<str>>(pattern: S) -> Option<BMLatin1> {
        let pattern = pattern.as_ref();

        let bad_char_shift_map = BMLatin1BadCharShiftMap::create_bad_char_shift_map_from_str(pattern)?;
        let bad_char_shift_map_rev = BMLatin1BadCharShiftMapRev::create_bad_char_shift_map_from_str(pattern)?;

        Some(BMLatin1 {
            bad_char_shift_map,
            bad_char_shift_map_rev,
            pattern: pattern.bytes().collect(),
        })
    }

    pub fn from_char_slice(pattern: &[char]) -> Option<BMLatin1> {
        let bad_char_shift_map = BMLatin1BadCharShiftMap::create_bad_char_shift_map_from_char_slice(pattern)?;
        let bad_char_shift_map_rev = BMLatin1BadCharShiftMapRev::create_bad_char_shift_map_from_char_slice(pattern)?;

        Some(BMLatin1 {
            bad_char_shift_map,
            bad_char_shift_map_rev,
            pattern: pattern.iter().map(|&c| c as u8).collect(),
        })
    }
}

impl BMLatin1 {
    pub fn find_all_in_u8_slice<B: ?Sized + AsRef<[u8]>>(&self, text: &B) -> Vec<usize> {
        find_in_u8_slice(text, &self.pattern, &self.bad_char_shift_map, 0)
    }

    pub fn find_all_in_str<S: AsRef<str>>(&self, text: S) -> Vec<usize> {
        find_in_str(text, &self.pattern, &self.bad_char_shift_map, 0)
    }

    pub fn find_all_in_char_slice(&self, text: &[char]) -> Vec<usize> {
        find_in_char_slice(text, &self.pattern, &self.bad_char_shift_map, 0)
    }

    pub fn find_first_in_u8_slice<B: ?Sized + AsRef<[u8]>>(&self, text: &B) -> Option<usize> {
        find_in_u8_slice(text, &self.pattern, &self.bad_char_shift_map, 1).get(0).map(|&p| p)
    }

    pub fn find_first_in_str<S: AsRef<str>>(&self, text: S) -> Option<usize> {
        find_in_str(text, &self.pattern, &self.bad_char_shift_map, 1).get(0).map(|&p| p)
    }

    pub fn find_first_in_char_slice(&self, text: &[char]) -> Option<usize> {
        find_in_char_slice(text, &self.pattern, &self.bad_char_shift_map, 1).get(0).map(|&p| p)
    }

    pub fn find_in_u8_slice<B: ?Sized + AsRef<[u8]>>(&self, text: &B, limit: usize) -> Vec<usize> {
        find_in_u8_slice(text, &self.pattern, &self.bad_char_shift_map, limit)
    }

    pub fn find_in_str<S: AsRef<str>>(&self, text: S, limit: usize) -> Vec<usize> {
        find_in_str(text, &self.pattern, &self.bad_char_shift_map, limit)
    }

    pub fn find_in_char_slice(&self, text: &[char], limit: usize) -> Vec<usize> {
        find_in_char_slice(text, &self.pattern, &self.bad_char_shift_map, limit)
    }
}

impl BMLatin1 {
    pub fn rfind_all_in_u8_slice<B: ?Sized + AsRef<[u8]>>(&self, text: &B) -> Vec<usize> {
        rfind_in_u8_slice(text, &self.pattern, &self.bad_char_shift_map_rev, 0)
    }

    pub fn rfind_all_in_str<S: AsRef<str>>(&self, text: S) -> Vec<usize> {
        rfind_in_str(text, &self.pattern, &self.bad_char_shift_map_rev, 0)
    }

    pub fn rfind_all_in_char_slice(&self, text: &[char]) -> Vec<usize> {
        rfind_in_char_slice(text, &self.pattern, &self.bad_char_shift_map_rev, 0)
    }

    pub fn rfind_first_in_u8_slice<B: ?Sized + AsRef<[u8]>>(&self, text: &B) -> Option<usize> {
        rfind_in_u8_slice(text, &self.pattern, &self.bad_char_shift_map_rev, 1).get(0).map(|&p| p)
    }

    pub fn rfind_first_in_str<S: AsRef<str>>(&self, text: S) -> Option<usize> {
        rfind_in_str(text, &self.pattern, &self.bad_char_shift_map_rev, 1).get(0).map(|&p| p)
    }

    pub fn rfind_first_in_char_slice(&self, text: &[char]) -> Option<usize> {
        rfind_in_char_slice(text, &self.pattern, &self.bad_char_shift_map_rev, 1).get(0).map(|&p| p)
    }

    pub fn rfind_in_u8_slice<B: ?Sized + AsRef<[u8]>>(&self, text: &B, limit: usize) -> Vec<usize> {
        rfind_in_u8_slice(text, &self.pattern, &self.bad_char_shift_map_rev, limit)
    }

    pub fn rfind_in_str<S: AsRef<str>>(&self, text: S, limit: usize) -> Vec<usize> {
        rfind_in_str(text, &self.pattern, &self.bad_char_shift_map_rev, limit)
    }

    pub fn rfind_in_char_slice(&self, text: &[char], limit: usize) -> Vec<usize> {
        rfind_in_char_slice(text, &self.pattern, &self.bad_char_shift_map_rev, limit)
    }
}

pub fn find_in_u8_slice<B: ?Sized + AsRef<[u8]>, P: ?Sized + AsRef<[u8]>>(text: &B, pattern: &P, bad_char_shift_map: &BMLatin1BadCharShiftMap, limit: usize) -> Vec<usize> {
    let text = text.as_ref();
    let pattern = pattern.as_ref();

    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;

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
                shift += bad_char_shift_map.t[text[shift + pattern_len_dec] as usize].max(
                    {
                        let c = text[p];

                        if c == last_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.t[c as usize] + 1
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

        shift += bad_char_shift_map.t[text[shift + pattern_len_dec] as usize].max(
            {
                let c = text[shift + pattern_len];

                if c == last_pattern_char {
                    1
                } else {
                    bad_char_shift_map.t[c as usize] + 1
                }
            }
        );
        if shift > end_index {
            break;
        }
    }

    result
}

pub fn find_in_str<S: AsRef<str>, P: ?Sized + AsRef<[u8]>>(text: S, pattern: &P, bad_char_shift_map: &BMLatin1BadCharShiftMap, limit: usize) -> Vec<usize> {
    let text = text.as_ref();

    find_in_u8_slice(text, pattern, bad_char_shift_map, limit)
}

pub fn find_in_char_slice<P: ?Sized + AsRef<[u8]>>(text: &[char], pattern: &P, bad_char_shift_map: &BMLatin1BadCharShiftMap, limit: usize) -> Vec<usize> {
    let text = text.as_ref();
    let pattern = pattern.as_ref();

    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;

    let last_pattern_char = pattern[pattern_len_dec];

    let mut shift = 0;

    let end_index = text_len - pattern_len;

    let mut result = vec![];

    'outer: loop {
        for (i, &pc) in pattern.iter().enumerate().rev() {
            if text[shift + i] as u8 != pc {
                let p = shift + pattern_len;
                if p == text_len {
                    break 'outer;
                }
                shift += bad_char_shift_map.t[text[shift + pattern_len_dec] as usize].max(
                    {
                        let c = text[p] as u8;

                        if c == last_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.t[c as usize] + 1
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

        shift += bad_char_shift_map.t[text[shift + pattern_len_dec] as usize].max(
            {
                let c = text[shift + pattern_len] as u8;

                if c == last_pattern_char {
                    1
                } else {
                    bad_char_shift_map.t[c as usize] + 1
                }
            }
        );
        if shift > end_index {
            break;
        }
    }

    result
}

pub fn rfind_in_u8_slice<B: ?Sized + AsRef<[u8]>, P: ?Sized + AsRef<[u8]>>(text: &B, pattern: &P, bad_char_shift_map: &BMLatin1BadCharShiftMapRev, limit: usize) -> Vec<usize> {
    let text = text.as_ref();
    let pattern = pattern.as_ref();

    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;

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
                shift -= bad_char_shift_map.t[text[shift - pattern_len_dec] as usize].max(
                    {
                        let c = text[shift - pattern_len];

                        if c == first_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.t[c as usize] + 1
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

        shift -= bad_char_shift_map.t[text[shift - pattern_len_dec] as usize].max(
            {
                let c = text[shift - pattern_len];

                if c == first_pattern_char {
                    1
                } else {
                    let s = bad_char_shift_map.t[c as usize] + 1;

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

pub fn rfind_in_str<S: AsRef<str>, P: ?Sized + AsRef<[u8]>>(text: S, pattern: &P, bad_char_shift_map: &BMLatin1BadCharShiftMapRev, limit: usize) -> Vec<usize> {
    let text = text.as_ref();

    rfind_in_u8_slice(text, pattern, bad_char_shift_map, limit)
}

pub fn rfind_in_char_slice<P: ?Sized + AsRef<[u8]>>(text: &[char], pattern: &P, bad_char_shift_map: &BMLatin1BadCharShiftMapRev, limit: usize) -> Vec<usize> {
    let text = text.as_ref();
    let pattern = pattern.as_ref();

    let text_len = text.len();
    let pattern_len = pattern.len();

    if text_len == 0 || pattern_len == 0 || text_len < pattern_len {
        return vec![];
    }

    let pattern_len_dec = pattern_len - 1;

    let first_pattern_char = pattern[0];

    let mut shift = text_len - 1;

    let start_index = pattern_len_dec;

    let mut result = vec![];

    'outer: loop {
        for (i, &pc) in pattern.iter().enumerate() {
            if text[shift - pattern_len_dec + i] as u8 != pc {
                if shift < pattern_len {
                    break 'outer;
                }
                shift -= bad_char_shift_map.t[text[shift - pattern_len_dec] as usize].max(
                    {
                        let c = text[shift - pattern_len] as u8;

                        if c == first_pattern_char {
                            1
                        } else {
                            bad_char_shift_map.t[c as usize] + 1
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

        shift -= bad_char_shift_map.t[text[shift - pattern_len_dec] as usize].max(
            {
                let c = text[shift - pattern_len] as u8;

                if c == first_pattern_char {
                    1
                } else {
                    let s = bad_char_shift_map.t[c as usize] + 1;

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