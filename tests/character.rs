extern crate boyer_moore_magiclen;
extern crate scanner_rust;

use boyer_moore_magiclen::*;

use scanner_rust::Scanner;

#[cfg(windows)]
const INPUT_DATA_PATH: &'static str = r"tests\data\character.txt";

#[cfg(not(windows))]
const INPUT_DATA_PATH: &'static str = r"tests/data/character.txt";

#[test]
fn data_input_from_file() {
    let mut sc = Scanner::scan_path(INPUT_DATA_PATH).unwrap();

    let pattern_count = sc.next_usize().unwrap().unwrap();

    for _ in 0..pattern_count {
        sc.skip_whitespaces().unwrap();

        let pattern = sc.next_line().unwrap().unwrap();

        let text_count = sc.next_usize().unwrap().unwrap();

        for _ in 0..text_count {
            sc.skip_whitespaces().unwrap();

            let text = sc.next_line().unwrap().unwrap();

            let answer_count = sc.next_usize().unwrap().unwrap();

            let mut answer = Vec::with_capacity(answer_count);

            for _ in 0..answer_count {
                answer.push(sc.next_usize().unwrap().unwrap());
            }

            let pattern = pattern.chars().collect::<Vec<char>>();
            let text = text.chars().collect::<Vec<char>>();

            let bm = BMCharacter::from_char_slice(&pattern).unwrap();
            assert_eq!(answer.clone(), bm.find_all_in_char_slice(&text));
            assert_eq!(answer.iter().map(|&i| i).rev().collect::<Vec<usize>>(), bm.rfind_all_in_char_slice(&text));
        }
    }
}