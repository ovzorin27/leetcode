use std::collections::HashSet;

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}

struct Solution;

impl Solution {
    /*
    Runtime: 0 ms, faster than 100.00% of Rust online submissions for Generate Parentheses.
Memory Usage: 2.3 MB, less than 53.16% of Rust online submissions for Generate Parentheses.
     */
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::generate(n).into_iter().collect()
    }

    fn generate(n: i32) -> HashSet<String> {
        if n <= 0 {
            panic!("n <= 0");
        } else if n == 1 {
            let mut set = HashSet::new();
            set.insert("()".to_string());
            set
        } else {
            let mut new_set = HashSet::new();
            let set = Self::generate(n - 1);
            for ref s in set {
                new_set.insert(format!("({})", s));
                new_set.insert(format!("(){}", s));
                new_set.insert(format!("{}()", s));
                for (i, c) in s.as_bytes().iter().enumerate() {
                    if *c == b'(' {
                        new_set.insert(format!("{}((){}", &s[0..i], &s[i+1..]));
                    } else if *c == b')' {
                        new_set.insert(format!("{})(){}", &s[0..i], &s[i+1..]));
                    }
                }
            }
            new_set
        }
    }
}