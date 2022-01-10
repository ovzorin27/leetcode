use std::collections::HashMap;

fn main() {
    println!("res: {:?}", Solution::letter_combinations("23".to_string()));
}

struct Solution;

impl Solution {
    /*
    Runtime: 0 ms, faster than 100.00% of Rust online submissions for Letter Combinations of a Phone Number.
Memory Usage: 2 MB, less than 96.70% of Rust online submissions for Letter Combinations of a Phone Number.
     */
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits = digits.as_bytes();
        let s = Self::combine(digits);
        let strings: Vec<String> = s.into_iter().map(|f|String::from_utf8(f).unwrap()).collect();

        strings
    }

    fn combine(digits: &[u8]) -> Vec<Vec<u8>> {
        if digits.is_empty() {
            vec![]
        } else if digits.len() == 1 {
            let mut new_vec = vec![];
            for v in Self::get_letters(digits[0]) {
                new_vec.push(vec![v]);
            }
            new_vec
        } else {
            let mut new_vec = vec![];
            let letters = Self::get_letters(digits[0]);
            let vec = Self::combine(&digits[1..]);
            for v in vec.iter() {
                for l in letters.iter() {
                    let mut cloned = v.clone();
                    cloned.insert(0, *l);
                    new_vec.push(cloned);
                }
            }

            new_vec
        }
    }

    fn get_letters(digit: u8) -> Vec<u8> {
        match digit {
            b'2' => vec![b'a', b'b', b'c'],
            b'3' => vec![b'd', b'e', b'f'],
            b'4' => vec![b'g', b'h', b'i'],
            b'5' => vec![b'j', b'k', b'l'],
            b'6' => vec![b'm', b'n', b'o'],
            b'7' => vec![b'p', b'q', b'r', b's'],
            b'8' => vec![b't', b'u', b'v'],
            b'9' => vec![b'w', b'x', b'y', b'z'],
            _ => panic!("wrong digit")
        }
    }
}