extern crate boyer_moore_magiclen;

extern crate needle;

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
                                  let bm = BMLatin1::from(pattern).unwrap();

                                  assert_eq!(answer.clone(), bm.find_full_all_in(text));
                                  assert_eq!(answer.iter().rev().map(|&n| n).collect::<Vec<usize>>(), bm.rfind_full_all_in(&text));
                                  assert_eq!(answer_not_full.clone(), bm.find_all_in(&text));
                                  assert_eq!(answer_not_full_rev.clone(), bm.rfind_all_in(&text));
                              },
    );
}