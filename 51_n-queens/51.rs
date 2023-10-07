//start/
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = vec![];
        let mut path = vec![vec!['.'; n as usize]; n as usize];
        let mut used = vec![false; n as usize];
        Self::back_tracking(n as usize, &mut res, &mut path, 0, &mut used);

        res
    }
    fn back_tracking(
        n: usize,
        res: &mut Vec<Vec<String>>,
        path: &mut Vec<Vec<char>>,
        start_index: usize,
        used: &mut Vec<bool>,
    ) {
        if start_index == n {
            res.push(
                path.iter()
                    .map(|v| v.iter().collect())
                    .collect(),
            );
            return;
        }
        for i in 0..n {
            if used[i] {
                continue;
            }
            if !Self::vaild(path, start_index, i, n) {
                continue;
            }

            used[i] = true;
            path[start_index][i] = 'Q';
            Self::back_tracking(n, res, path, start_index + 1, used);
            path[start_index][i] = '.';
            used[i] = false;
        }
    }

    fn vaild(path: &[Vec<char>], start_index: usize, col: usize, n: usize) -> bool {
        for j in 1..n {
            if j > start_index || j > col {
                break;
            }
            let row = start_index - j;
            let col = col - j;
            if path[row][col] == 'Q' {
                return false;
            }
        }
        for j in 1..n {
            let col = col + j;
            if j > start_index || col >= n {
                break;
            }
            let row = start_index - j;
            if path[row][col] == 'Q' {
                return false;
            }
        }
        true
    }
}
//end/
struct Solution;
fn main() {
    // println!("{:#?}", Solution::solve_n_queens(4));
    println!("{:#?}", Solution::solve_n_queens(15).len());
    // println!("{:#?}", Solution::solve_n_queens(3));
    // println!("{:#?}", Solution::solve_n_queens(1));
    // println!("{:#?}", Solution::solve_n_queens(100).len());
}
