use std::collections::HashMap;

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    assert_eq!(Solution2::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
}

struct Solution;
struct Solution2;

/*
Runtime: 20 ms, faster than 35.99% of Rust online submissions for Two Sum.
Memory Usage: 2.2 MB, less than 78.02% of Rust online submissions for Two Sum.
 */
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        
        return vec![];
    }
}

/*
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Two Sum.
Memory Usage: 2.3 MB, less than 54.49% of Rust online submissions for Two Sum.
 */
impl Solution2 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut elements: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            match elements.get(&nums[i]) {
                Some( &index) => return vec![index, i as i32],
                None => {
                    let diff = target - nums[i];
                    elements.insert(diff, i as i32)
                },
            };
        }

        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), [1, 2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), [0, 1]);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution2::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution2::two_sum(vec![3, 2, 4], 6), [1, 2]);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution2::two_sum(vec![3, 3], 6), [0, 1]);
    }
}