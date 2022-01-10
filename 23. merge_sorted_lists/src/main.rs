fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 5,
                next: None
            }))
        }))
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 5,
                next: None
            }))
        }))
    }));
    let l3 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 6,
            next: None
        }))
    }));
    let vec = vec![l1, l2, l3];
    let res = Solution::merge_k_lists(vec);
    println!("{:?}", res);
}

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

struct Solution;

impl Solution {
    /*
    Runtime: 220 ms, faster than 17.86% of Rust online submissions for Merge k Sorted Lists.
Memory Usage: 3.9 MB, less than 5.95% of Rust online submissions for Merge k Sorted Lists.
     */
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut min = i32::MAX;
        let mut num = 0;
        for (index, element) in lists.iter().enumerate() {
            if let Some(node) = element {
                if node.val < min {
                    min = node.val;
                    num = index;
                }
            }
        }

        if min == i32::MAX {
            return None;
        }

        for (index, element) in lists.iter_mut().enumerate() {
            if index == num {
                let node = element.take();
                *element = node.unwrap().next;
            }
        }

        Some(Box::new(ListNode {
            val: min,
            next: Self::merge_k_lists(lists)
        }))
    }
}