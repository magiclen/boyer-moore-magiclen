extern crate boyer_moore_magiclen;

mod lib;

use boyer_moore_magiclen::*;

#[cfg(windows)]
const INPUT_DATA_PATH: &'static str = r"tests\data\latin_1.txt";

#[cfg(not(windows))]
const INPUT_DATA_PATH: &'static str = r"tests/data/latin_1.txt";

#[test]
fn data_input_from_file() {
    lib::data_input_from_file(INPUT_DATA_PATH,
                              |text, pattern, answer, answer_not_full, answer_not_full_rev| {
                                  let bm = BMLatin1::from_u8_slice(&pattern).unwrap();

                                  assert_eq!(answer.clone(), bm.find_full_all_in_u8_slice(&text));
                                  assert_eq!(answer.iter().rev().map(|&n| n).collect::<Vec<usize>>(), bm.rfind_full_all_in_u8_slice(&text));
                                  assert_eq!(answer_not_full.clone(), bm.find_all_in_u8_slice(&text));
                                  assert_eq!(answer_not_full_rev.clone(), bm.rfind_all_in_u8_slice(&text));

                                  let bm = BMLatin1::from_str(&pattern).unwrap();

                                  assert_eq!(answer.clone(), bm.find_full_all_in_str(&text));
                                  assert_eq!(answer.iter().rev().map(|&n| n).collect::<Vec<usize>>(), bm.rfind_full_all_in_str(&text));
                                  assert_eq!(answer_not_full.clone(), bm.find_all_in_str(&text));
                                  assert_eq!(answer_not_full_rev.clone(), bm.rfind_all_in_str(&text));

                                  let pattern = pattern.chars().collect::<Vec<char>>();
                                  let text = text.chars().collect::<Vec<char>>();

                                  let bm = BMLatin1::from_char_slice(&pattern).unwrap();

                                  assert_eq!(answer.clone(), bm.find_full_all_in_char_slice(&text));
                                  assert_eq!(answer.iter().rev().map(|&n| n).collect::<Vec<usize>>(), bm.rfind_full_all_in_char_slice(&text));
                                  assert_eq!(answer_not_full.clone(), bm.find_all_in_char_slice(&text));
                                  assert_eq!(answer_not_full_rev.clone(), bm.rfind_all_in_char_slice(&text));
                              },
    );
}