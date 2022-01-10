//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn main() {
    //l1 = [2,4,3], l2 = [5,6,4]
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode::new(3)))}))
    }));
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode::new(4)))}))
    }));

    let list = Solution::add_two_numbers2(Solution::get_list_from_number(999),
    Solution::get_list_from_number(999));
    println!("list = {:?}", list);
}

struct Solution;

impl Solution {
    /*
    Runtime: 4 ms, faster than 78.57% of Rust online submissions for Add Two Numbers.
Memory Usage: 2 MB, less than 99.21% of Rust online submissions for Add Two Numbers.
     */
    pub fn add_two_numbers2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add_two_numbers_recursive(l1.as_ref(), l2.as_ref(), 0)
    }

    pub fn add_two_numbers_recursive(l1_ref: Option<&Box<ListNode>>, l2_ref: Option<&Box<ListNode>>, overflow: i32) -> Option<Box<ListNode>> {
        match (l1_ref, l2_ref) {
            (Some(l1), Some(l2)) => {
                let sum = l1.val + l2.val + overflow;
                Some(Box::new(ListNode { val: sum % 10, next: Self::add_two_numbers_recursive(l1.next.as_ref(), l2.next.as_ref(), sum / 10) }))
            },
            (Some(l1), None) => {
                let sum = l1.val + overflow;
                Some(Box::new(ListNode { val: sum % 10, next: Self::add_two_numbers_recursive(l1.next.as_ref(), None, sum / 10) }))
            },
            (None, Some(l2)) => {
                let sum = l2.val + overflow;
                Some(Box::new(ListNode { val: sum % 10, next: Self::add_two_numbers_recursive(None, l2.next.as_ref(), sum / 10) }))
            },
            (None, None) if overflow == 1 => {
                Some(Box::new(ListNode { val: 1, next: None }))
            }
            (_, _) => None
        }
    }

    /*
    Runtime: 4 ms
Memory Usage: 2.3 MB
     */
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1_ref = &l1;
        let mut l2_ref = &l2;
        let mut overflow = 0;
        let mut list: Option<Box<ListNode>> = None;

        let mut l1_end = false;
        let mut l2_end = false;

        loop {
            let mut l1_value = 0;
            let mut l2_value = 0;

            if let Some(v) = l1_ref {
                l1_value = v.val;
                l1_ref = &v.next;
            } else {
                l1_value = 0;
                l1_end = true;
            }
            if let Some(v) = l2_ref {
                l2_value = v.val;
                l2_ref = &v.next;
            } else {
                l2_value = 0;
                l2_end = true
            }

            if l1_end && l2_end {
                if overflow > 0 {
                    let mut list_ref = &mut list;
                    while let Some(v) = list_ref {
                        if v.next.is_none() {
                            list_ref.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
                            break;
                        } else {
                            list_ref = &mut list_ref.as_mut().unwrap().next;
                        }
                    }
                }
                return list;
            } else {
                let mut sum = l1_value + l2_value + overflow;
                if sum > 9 {
                    sum = sum % 10;
                    overflow = 1;
                } else {
                    overflow = 0;
                }

                if list.is_none() {
                    list = Some(Box::new(ListNode::new(sum as i32)));
                } else {
                    let mut list_ref = &mut list;
                    while let Some(v) = list_ref {
                        if v.next.is_none() {
                            list_ref.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum as i32)));
                            break;
                        } else {
                            list_ref = &mut list_ref.as_mut().unwrap().next;
                        }
                    }
                }
            }
        }
    }

    // [2,4,3] -> 342
    pub fn get_number_from_list(list: Option<Box<ListNode>>) -> i64 {
        let mut current_node = &list;
        let mut sum = 0_i64;
        let mut index = 0;
        while let Some(node) = current_node {
            sum += node.val as i64 * 10_i64.pow(index);
            current_node = &node.next;
            index += 1;
        }

        println!("sum = {}", sum);
        sum
    }

    // 342 -> [2,4,3]
    pub fn get_list_from_number(number: i64) -> Option<Box<ListNode>> {
        if number == 0 {
            return Some(Box::new(ListNode::new(0)));
        }

        let mut list: Option<Box<ListNode>> = None;
        let mut num = number;
        while num > 0 {
            let val = (num % 10);
            num = (num - val) / 10;
            if list.is_none() {
                list = Some(Box::new(ListNode::new(val as i32)));
            } else {
                let mut list_ref = &mut list;
                while let Some(v) = list_ref {
                    if v.next.is_none() {
                        list_ref.as_mut().unwrap().next = Some(Box::new(ListNode::new(val as i32)));
                        break;
                    } else {
                        list_ref = &mut list_ref.as_mut().unwrap().next;
                    }
                }
            }
        }

        println!("list = {:?}", list);
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::get_list_from_number(0), Some(Box::new(ListNode::new(0))));
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::get_list_from_number(4), Some(Box::new(ListNode::new(4))));
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::get_list_from_number(342), Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3)))}))
        })));
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::get_number_from_list(Some(Box::new(ListNode::new(0)))), 0);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::get_number_from_list(Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3)))}))
        }))), 342);
    }

    #[test]
    fn test_6() {
        let i = 9999999991_i64;
        assert_eq!(Solution::get_number_from_list(Solution::get_list_from_number(i)), i);
    }
}