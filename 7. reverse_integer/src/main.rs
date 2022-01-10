fn main() {
    Solution::reverse(123);
}

struct Solution;

impl Solution {
    /*
    Runtime: 0 ms, faster than 100.00% of Rust online submissions for Reverse Integer.
Memory Usage: 2 MB, less than 56.47% of Rust online submissions for Reverse Integer.
     */
    pub fn reverse(x: i32) -> i32 {
        if x >= -9 && x <= 9 {
            return x;
        }

        let mut val = x.abs();
        let mut sum: i32 = 0;
        let mut checked_sum: Option<i32> = None;
        let mut vec: Vec<u8> = Vec::with_capacity(3);

        while val > 0 {
            let digit = val % 10;
            val /= 10;
            vec.push(digit as u8);
        }

        for (step, digit) in vec.iter().rev().enumerate() {
            checked_sum = 10_i32.checked_pow(step as u32).and_then(
                |v| v.checked_mul(*digit as i32)
            ).and_then(
                |v|v.checked_add(sum)
            );
            if let Some(s) = checked_sum {
                sum = s;
            } else {
                return 0;
            }
        }

        if x < 0 {
            -sum
        } else {
            sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::reverse(-123), -321);
    }
}