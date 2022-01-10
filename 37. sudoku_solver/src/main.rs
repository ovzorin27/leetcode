fn main() {
    let mut vec = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    Solution::solve_sudoku(&mut vec);

    println!("result: ");
    for i in 0..9 {
        for j in 0..9 {
            print!("{} ", vec[i][j]);
        }
        print!("\n");
    }
}

use std::char::*;

struct Solution;

impl Solution {
    /*
Runtime: 8 ms, faster than 34.21% of Rust online submissions for Sudoku Solver.
Memory Usage: 2 MB, less than 63.16% of Rust online submissions for Sudoku Solver.
     */
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Solution::find(board, 0);
    }

    fn find(vec: &mut Vec<Vec<char>>, index: usize) -> bool {
        if index >= 81 {
            return true
        }

        let (i, j) = (index / 9, index % 9);
        let p_min = (i / 3) * 3;
        let q_min = (j / 3) * 3;

        if vec[i][j] == '.' {
            'outer:
            for val in 1..=9 {
                let val_char = std::char::from_digit(val, 10).unwrap();
                // check if 'val' is already exists
                // check inside small box
                for p in p_min..(p_min + 3) {
                    for q in q_min..(q_min + 3) {
                        if val_char == vec[p][q] {
                            continue 'outer;
                        }
                    }
                }

                // check for vertical line
                for p in 0..9 {
                    if val_char == vec[p][j] {
                        continue 'outer;
                    }
                }

                for q in 0..9 {
                    if val_char == vec[i][q] {
                        continue 'outer;
                    }
                }

                vec[i][j] = val_char;
                if Solution::find(vec, index + 1) {
                    return true;
                }
            }
            vec[i][j] = '.';
            return false;
        } else {
            Solution::find(vec, index + 1)
        }
    }
}