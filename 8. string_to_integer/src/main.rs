fn main() {
    println!("{}", Solution::my_atoi("   -0234 dsf".to_string()));
}

struct Solution;

impl Solution {
    /*
    Runtime: 0 ms, faster than 100.00% of Rust online submissions for String to Integer (atoi).
Memory Usage: 2.1 MB, less than 60.87% of Rust online submissions for String to Integer (atoi).
     */
    pub fn my_atoi(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let vec = s.as_bytes();

        let mut start = 0;
        for i in 0..vec.len() {
            if vec[i] != ' ' as u8 {
                start = i;
                break;
            } else {
                start += 1;
            }
        }

        if start >= vec.len() {
            return 0;
        }

        let is_negative = if vec[start] == '-' as u8 {
            start += 1;
            true
        } else if vec[start] == '+' as u8 {
            start += 1;
            false
        } else {
            false
        };

        for i in start..vec.len() {
            if vec[i] != '0' as u8 {
                start = i;
                break;
            } else {
                start += 1;
            }
        }

        let mut end = vec.len();
        for i in start..vec.len() {
            if vec[i] < '0' as u8 || vec[i] > '9' as u8 {
                end = i;
                break;
            }
        }

        let mut sum: i32 = 0;
        for (count, index) in (start..(end)).rev().enumerate() {
            let d = vec[index] - ('0' as u8);

            let checked_sum = 10_i32.checked_pow(count as u32).and_then(
                |v|v.checked_mul(d as i32)
            ).and_then(
                |v|v.checked_add(sum)
            );

            if let Some(s) = checked_sum {
                sum = s;
            } else {
                return if is_negative {
                    i32::MIN
                } else {
                    i32::MAX
                }
            }
        }

        if is_negative { -sum } else { sum }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::my_atoi("   -0234 dsf".to_string()), -234);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::my_atoi("   -023445645645df dsf".to_string()), i32::MIN);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::my_atoi("   023445645645df dsf".to_string()), i32::MAX);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::my_atoi("  0000000000012345678".to_string()), 12345678);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::my_atoi("  000000000000000000".to_string()), 0);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::my_atoi("  ".to_string()), 0);
    }

    #[test]
    fn test_7() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }
}