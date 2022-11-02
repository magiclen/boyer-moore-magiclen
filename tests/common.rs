use std::path::Path;

use scanner_rust::Scanner;

#[allow(dead_code)]
pub(crate) fn data_input_from_file<
    P: AsRef<Path>,
    F: Fn(&str, &str, Vec<usize>, Vec<usize>, Vec<usize>),
>(
    p: P,
    f: F,
) {
    let mut sc = Scanner::scan_path(p).unwrap();

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

            let pattern_length = pattern.len();

            let mut answer_not_full = Vec::with_capacity(answer.len());

            let mut min_index = 0;

            for &index in answer.iter() {
                if index >= min_index {
                    answer_not_full.push(index);

                    min_index = index + pattern_length;
                }
            }

            let mut answer_not_full_rev = Vec::with_capacity(answer.len());

            let mut max_index = text.len();

            for &index in answer.iter().rev() {
                if index <= max_index {
                    answer_not_full_rev.push(index);

                    if index > pattern_length {
                        max_index = index - pattern_length;
                    } else {
                        break;
                    }
                }
            }

            f(&text, &pattern, answer, answer_not_full, answer_not_full_rev)
        }
    }
}
