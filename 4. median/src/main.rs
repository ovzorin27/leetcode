fn main() {
    println!(" res = {}", Solution::find_median_sorted_arrays(vec![1, 2], vec![-1, 3]));
}

struct Solution;

impl Solution {
    /*
    Runtime: 0 ms, faster than 100.00% of Rust online submissions for Median of Two Sorted Arrays.
Memory Usage: 2 MB, less than 63.91% of Rust online submissions for Median of Two Sorted Arrays.
     */
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        fn get_median(vec: &[i32]) -> f64 {
            let i = vec.len() / 2;
            if vec.len() % 2 == 0 {
                (vec[i] - vec[i - 1]).abs() as f64 / 2.0 + vec[i].min(vec[i - 1]) as f64
            } else {
                vec[i] as f64
            }
        }

        fn insert(arr: &mut [i32], val: i32) {
            if arr[0] < arr[1] {
                arr[0] = val;
            } else {
                arr[1] = val;
            }
        }

        let mut arr = [i32::MIN, i32::MIN];
        let l1_len = nums1.len();
        let l2_len = nums2.len();
        let l = l1_len + l2_len;

        if l1_len == 0 {
            return get_median(&nums2[..]);
        }
        if l2_len == 0 {
            return get_median(&nums1[..]);
        }

        let mut l1_idx = 0;
        let mut l2_idx = 0;
        for i in 0..(l/2 + 1) {
            match (l1_idx, l2_idx) {
                (i1, i2) if i1 < l1_len && i2 < l2_len => {
                    if nums1[i1] < nums2[i2] {
                        insert(&mut arr, nums1[i1]);
                        l1_idx += 1;
                    } else {
                        insert(&mut arr, nums2[i2]);
                        l2_idx += 1;
                    }
                },
                (i1, i2) if i1 < l1_len && i2 == l2_len => {
                    insert(&mut arr, nums1[i1]);
                    l1_idx += 1;
                },
                (i1, i2) if i1 == l1_len && i2 < l2_len => {
                    insert(&mut arr, nums2[i2]);
                    l2_idx += 1;
                }
                _ => { }
            }
        }

        if l % 2 == 0 {
            (arr[0] - arr[1]).abs() as f64 / 2.0 + arr[0].min(arr[1]) as f64
        } else {
            arr[0].max(arr[1]) as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2_f64);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3, 5], vec![2, 4]), 3_f64);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5_f64);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![]), 2_f64);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4], vec![]), 2.5_f64);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![3], vec![-2, -1]), -1_f64);
    }

    #[test]
    fn test_7() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![-1, 3]), 1.5_f64);
    }
}