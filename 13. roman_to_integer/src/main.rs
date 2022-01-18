fn main() {
    println!("{}", Solution::roman_to_int("MCMXCIV".to_string()));
}

struct Solution;

impl Solution {
    /*
    Runtime: 12 ms, faster than 13.95% of Rust online submissions for Roman to Integer.
Memory Usage: 2.2 MB, less than 28.20% of Rust online submissions for Roman to Integer.
     */
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut iter = s.as_bytes().iter();
        let mut cur = iter.next();
        let mut next = iter.next();
        loop {
            println!(" cur {:?}, next {:?}", cur, next);
            match (cur, next) {
                (None, None) => break,
                (Some(b'I'), Some(b'V')) => {
                    sum += 4;
                    cur = iter.next();
                    next = iter.next();
                },
                (Some(b'I'), Some(b'X')) => {
                    sum += 9;
                    cur = iter.next();
                    next = iter.next();
                },
                (Some(b'X'), Some(b'L')) => {
                    sum += 40;
                    cur = iter.next();
                    next = iter.next();
                },
                (Some(b'X'), Some(b'C')) => {
                    sum += 90;
                    cur = iter.next();
                    next = iter.next();
                },
                (Some(b'C'), Some(b'D')) => {
                    sum += 400;
                    cur = iter.next();
                    next = iter.next();
                },
                (Some(b'C'), Some(b'M')) => {
                    sum += 900;
                    cur = iter.next();
                    next = iter.next();
                },
                (Some(b'I'), _) => {
                    sum += 1;
                    cur = next;
                    next = iter.next();
                },
                (Some(b'V'), _) => {
                    sum += 5;
                    cur = next;
                    next = iter.next();
                },
                (Some(b'X'), _) => {
                    sum += 10;
                    cur = next;
                    next = iter.next();
                },
                (Some(b'L'), _) => {
                    sum += 50;
                    cur = next;
                    next = iter.next();
                },
                (Some(b'C'), _) => {
                    sum += 100;
                    cur = next;
                    next = iter.next();
                },
                (Some(b'D'), _) => {
                    sum += 500;
                    cur = next;
                    next = iter.next();
                },
                (Some(b'M'), _) => {
                    sum += 1000;
                    cur = next;
                    next = iter.next();
                },
                _ => panic!("not valid")
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}