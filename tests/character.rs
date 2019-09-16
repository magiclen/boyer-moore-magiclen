#![cfg(feature = "character")]

extern crate boyer_moore_magiclen;

mod lib;

use boyer_moore_magiclen::*;

const INPUT_DATA_PATH: &str = r"tests/data/character.txt";

#[test]
fn data_input_from_file() {
    lib::data_input_from_file(
        INPUT_DATA_PATH,
        |text, pattern, answer, answer_not_full, answer_not_full_rev| {
            let pattern = pattern.chars().collect::<Vec<char>>();
            let text = text.chars().collect::<Vec<char>>();

            let bm = BMCharacter::from(&pattern).unwrap();

            assert_eq!(answer.clone(), bm.find_full_all_in(&text));
            assert_eq!(
                answer.iter().rev().copied().collect::<Vec<usize>>(),
                bm.rfind_full_all_in(&text)
            );
            assert_eq!(answer_not_full.clone(), bm.find_all_in(&text));
            assert_eq!(answer_not_full_rev.clone(), bm.rfind_all_in(&text));
        },
    );
}
