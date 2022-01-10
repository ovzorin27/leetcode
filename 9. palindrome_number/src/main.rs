fn main() {
    Solution::is_palindrome(121);
}

struct Solution;

impl Solution {
    /*
    Runtime: 18 ms, faster than 13.13% of Rust online submissions for Palindrome Number.
Memory Usage: 2.1 MB, less than 66.27% of Rust online submissions for Palindrome Number.
     */
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        let bytes = s.as_bytes();
        let len = bytes.len();
        for i in 0..(len / 2) {
            if bytes[i] != bytes[len - i - 1_usize] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        assert_eq!(Solution::is_palindrome(12321), true);
    }

    #[test]
    pub fn test2() {
        assert_eq!(Solution::is_palindrome(1251), false);
    }
}