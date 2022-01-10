fn main() {
    println!("{:?}", Solution::search_range(vec![1, 2, 3], 1));
}

struct Solution;

impl Solution {
    /*
    Runtime: 0 ms, faster than 100.00% of Rust online submissions for Find First and Last Position of Element in Sorted Array.
Memory Usage: 2.3 MB, less than 93.75% of Rust online submissions for Find First and Last Position of Element in Sorted Array.
     */
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut mid = 0;

        if nums[0] > target || nums[right] < target {
            return vec![-1, -1];
        }

        while left <= right {
            mid = (left + right) / 2;
            if nums[mid] == target {
                break;
            }
            if target < nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        if nums[mid] != target {
            return vec![-1, -1];
        }

        left = Self::search_left(&nums, target, left, right);
        right = Self::search_right(&nums, target, left, right);

        vec![left as i32, right as i32]
    }

    fn search_left(nums: &[i32], target: i32, mut left: usize, mut right: usize, ) -> usize {
        let mut mid = 0;
        while left + 1 < right {
            mid = (left + right) / 2;
            if nums[mid] != target {
                left = mid;
            } else {
                right = mid
            }
        }

        if nums[left] == target {
            left
        } else {
            right
        }
    }

    fn search_right(nums: &[i32], target: i32, mut left: usize, mut right: usize, ) -> usize {
        let mut mid = 0;
        while left + 1 < right {
            mid = (left + right) / 2;
            if nums[mid] != target {
                right = mid;
            } else {
                left = mid
            }
        }

        if nums[right] == target {
            right
        } else {
            left
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        assert_eq!(Solution::search_range(vec![1, 2, 3, 4, 6, 7, 8, 8, 8, 9, 10, 11], 8), vec![6, 8]);
    }

    #[test]
    pub fn test2() {
        assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0]);
    }

    #[test]
    pub fn test3() {
        assert_eq!(Solution::search_range(vec![1], 8), vec![-1, -1]);
    }

    #[test]
    pub fn test4() {
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }

    #[test]
    pub fn test5() {
        assert_eq!(Solution::search_range(vec![1, 2, 3], 1), vec![0, 0]);
    }
}