fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    /*
    Runtime: 0 ms, faster than 100.00% of Rust online submissions for Valid Parentheses.
Memory Usage: 2.1 MB, less than 31.49% of Rust online submissions for Valid Parentheses.
     */
    pub fn is_valid(s: String) -> bool {
        let mut vec: Vec<u8> = vec![];
        for c in s.as_bytes() {
            match c {
                b'(' | b'{' | b'[' => {
                    vec.push(*c);
                },
                b')' => {
                    if let Some(c) = vec.pop() {
                        if c != b'(' { return false }
                    } else {
                        return false;
                    }
                },
                b'}' => {
                    if let Some(c) = vec.pop() {
                        if c != b'{' { return false }
                    } else {
                        return false;
                    }
                },
                b']' => {
                    if let Some(c) = vec.pop() {
                        if c != b'[' { return false }
                    } else {
                        return false;
                    }
                },
                _ => panic!("wrong symbol")
            }
        }

        vec.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_valid("()({}{}[])".to_string()), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_valid("()({(}){}[])".to_string()), false);
    }
}