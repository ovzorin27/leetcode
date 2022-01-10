fn main() {
    println!("Hello, world!");
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
    println!("{}", Solution::is_valid_sudoku(vec));
}

struct Solution;

impl Solution {
    /*
    Runtime: 4 ms, faster than 81.52% of Rust online submissions for Valid Sudoku.
Memory Usage: 2 MB, less than 97.83% of Rust online submissions for Valid Sudoku.
     */
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        Self::find(&board, 0)
    }

    fn find(vec: &Vec<Vec<char>>, index: usize) -> bool {
        if index >= 81 {
            return true
        }

        let (i, j) = (index / 9, index % 9);
        let p_min = (i / 3) * 3;
        let q_min = (j / 3) * 3;

        if vec[i][j] != '.' {
            let val_char = vec[i][j];
            // check if 'val' is already exists
            // check inside small box
            for p in p_min..(p_min + 3) {
                for q in q_min..(q_min + 3) {
                    if val_char == vec[p][q] && p != i {
                        return false;
                    }
                }
            }

            // check for vertical line
            for p in 0..9 {
                if val_char == vec[p][j] && p != i {
                    return false;
                }
            }

            for q in 0..9 {
                if val_char == vec[i][q] && q != j {
                    return false
                }
            }

            if Solution::find(vec, index + 1) {
                true
            } else {
                false
            }
        } else {
            Solution::find(vec, index + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let board = vec![
            vec![".","8","7","6","5","4","3","2","1"],
            vec!["2",".",".",".",".",".",".",".","."],
            vec!["3",".",".",".",".",".",".",".","."],
            vec!["4",".",".",".",".",".",".",".","."],
            vec!["5",".",".",".",".",".",".",".","."],
            vec!["6",".",".",".",".",".",".",".","."],
            vec!["7",".",".",".",".",".",".",".","."],
            vec!["8",".",".",".",".",".",".",".","."],
            vec!["9",".",".",".",".",".",".",".","."]
        ].iter().map(|f|f.iter().map(|f|char::from(f.as_bytes()[0])).collect()).collect();
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }
}