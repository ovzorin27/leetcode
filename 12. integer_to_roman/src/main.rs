fn main() {
    for i in 0..=99 {
        println!("{}: {}", i, Solution::int_to_roman(i));
    }
}

struct Solution;

impl Solution {
    /*
    Runtime: 3 ms, faster than 62.07% of Rust online submissions for Integer to Roman.
Memory Usage: 2 MB, less than 57.76% of Rust online submissions for Integer to Roman.
     */
    pub fn int_to_roman(num: i32) -> String {
        let mut s = String::new();
        Self::convert(num, &mut s);

        s
    }

    fn convert(mut num: i32, s: &mut String) {
        match num {
            0 => return,
            1..=3 => {
                s.push_str("I");
                num -= 1;
            },
            4 => {
                s.push_str("IV");
                num -= 4;
            },
            5..=8 => {
                s.push_str("V");
                num -= 5;
            },
            9 => {
                s.push_str("IX");
                num -= 9;
            },
            10..=39 => {
                s.push_str("X");
                num -= 10;
            },
            40..=49 => {
                s.push_str("XL");
                num -= 40;
            },
            50..=89 => {
                s.push_str("L");
                num -= 50;
            }
            90..=99 => {
                s.push_str("XC");
                num -= 90;
            },
            100..=399 => {
                s.push_str("C");
                num -= 100;
            },
            400..=499 => {
                s.push_str("CD");
                num -= 400;
            },
            500..=899 => {
                s.push_str("D");
                num -= 500;
            },
            900..=999 => {
                s.push_str("CM");
                num -= 900;
            },
            1000..=3999 => {
                s.push_str("M");
                num -= 1000;
            }
            _ => {
                panic!("not valid number");
            }
        }

        Self::convert(num, s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::int_to_roman(354), "CCCLIV".to_string());
    }
}