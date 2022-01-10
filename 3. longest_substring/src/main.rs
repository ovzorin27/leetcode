use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    println!("{}", Solution::length_of_longest_substring("abcabcdea".to_string()));
}

struct Solution;

impl Solution {
    /*
Runtime: 134 ms, faster than 14.56% of Rust online submissions for Longest Substring Without Repeating Characters.
Memory Usage: 2.3 MB, less than 10.73% of Rust online submissions for Longest Substring Without Repeating Characters.
 */
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();

        let mut set_of_char: HashSet<char> = HashSet::new();
        let mut max_len: i32 = 0;
        for i in 0..chars.len() {
            if chars.len() - i < max_len as usize {
                return max_len;
            }

            set_of_char.clear();
            let mut current_len = 0;
            for j in i..chars.len() {
                if set_of_char.contains(&chars[j]) {
                    break;
                } else {
                    set_of_char.insert(chars[j]);
                    current_len += 1;
                }
            }
            if current_len > max_len {
                max_len = current_len;
            }
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::length_of_longest_substring("abcabcdea".to_string()), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::length_of_longest_substring("abcabc".to_string()), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_longest_substring("aaaaa".to_string()), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::length_of_longest_substring("aaaabc".to_string()), 3);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::length_of_longest_substring("abcabcab".to_string()), 3);
    }
}