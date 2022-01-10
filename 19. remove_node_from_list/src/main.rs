fn main() {
    println!("Hello, world!");
    let list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: None
                    }))
                }))
            }))
        }))
    }));
    println!("{:?}", Solution::remove_nth_from_end(list, 2));
}

// Definition for singly-linked list.
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
    Runtime: 0 ms, faster than 100.00% of Rust online submissions for Remove Nth Node From End of List.
Memory Usage: 2 MB, less than 80.25% of Rust online submissions for Remove Nth Node From End of List.
     */
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut head = head;
        let mut count = 0;
        let mut item = &mut head;
        while let Some(node) = item {
            println!(" {}", node.val);
            item = &mut node.next;
            count += 1;
        }

        if count - n <= 0 {
            return head.unwrap().next;
        }

        item = &mut head;
        for i in 0..(count - n - 1) {
            if let Some(v) = item {
                item = &mut v.next;
            }
        }

        if let Some(v) = item {
            let deleted_item = v.next.take();
            v.next = deleted_item.unwrap().next;
        } else {
            return None;
        }

        head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test1() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: None
        }));
        assert_eq!(Solution::remove_nth_from_end(list, 1), None);
    }


}