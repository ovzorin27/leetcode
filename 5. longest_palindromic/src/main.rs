fn main() {
    Solution::longest_palindrome("bb".to_string());
}

struct Solution;

impl Solution {
    /*
    Runtime: 1172 ms, faster than 5.34% of Rust online submissions for Longest Palindromic Substring.
Memory Usage: 2.3 MB, less than 18.22% of Rust online submissions for Longest Palindromic Substring.
     */
    pub fn longest_palindrome(s: String) -> String {
        fn is_palindrome(s: &[u8]) -> bool {
            let mut i = 0;
            let mut j = s.len() - 1;
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }

            true
        }

        let vec: Vec<u8> = s.bytes().collect();
        let len = vec.len();

        if len <= 1 {
            return s;
        }

        let mut start = 0;
        let mut end = 1;
        for i in 0..(len - 1) {
            for j in (i + 2)..(len + 1)  {
                if is_palindrome(&vec[i..j]) && (j - i > end - start) {
                    start = i;
                    end = j;
                }
            }
        }

        s[start..end].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a".to_string());
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::longest_palindrome("bb".to_string()), "bb".to_string());
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_palindrome("bbb".to_string()), "bbb".to_string());
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::longest_palindrome("ac".to_string()), "a".to_string());
    }
}