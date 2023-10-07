//start/
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::back_tracking(board);
    }
    fn back_tracking(board: &mut Vec<Vec<char>>) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] != '.' {
                    continue;
                }
                for num in '1'..='9' {
                    if !Self::is_vaild(board, row, col, &num) {
                        continue;
                    }
                    board[row][col] = num;
                    if Self::back_tracking(board) {
                        return true;
                    };
                    board[row][col] = '.';
                }
                return false;
            }
        }
        true
    }
    fn is_vaild(board: &[Vec<char>], row: usize, col: usize, num: &char) -> bool {
        // row
        if board[row].contains(num) {
            return false;
        }
        // column
        for i in board.iter() {
            if i[col] == *num {
                return false;
            }
        }
        let row = row / 3 * 3;
        let col = col / 3 * 3;

        for b_row in board.iter().skip(row).take(3) {
            for b_col in b_row.iter().skip(col).take(3) {
                if b_col == num {
                    return false;
                }
            }
        }

        // for i in row..row + 3 {
        //     for j in col..col + 3 {
        //         if board[i][j] == num {
        //             return false;
        //         }
        //     }
        // }

        true
    }
}
//end/
struct Solution;
fn main() {
    let mut one = vec![
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
    // for i in one.iter() {
    //     println!("{:?}", i);
    // }
    // println!("----");
    Solution::solve_sudoku(&mut one);
    for i in one.iter() {
        println!("{:?}", i);
    }
}
