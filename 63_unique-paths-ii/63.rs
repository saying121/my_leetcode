struct Solution;

//start/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            if obstacle_grid[i][0] == 1 {
                break;
            }
            dp[i][0] = 1;
        }
        for j in 0..n {
            if obstacle_grid[0][j] == 1 {
                break;
            }
            dp[0][j] = 1;
        }

        for i in 1..m {
            for j in 1..n {
                let left = obstacle_grid[i][j - 1];
                let left_val = dp[i][j - 1];
                let up = obstacle_grid[i - 1][j];
                let up_val = dp[i - 1][j];
                let self_block = obstacle_grid[i][j];
                match (left, up, self_block) {
                    (0, 0, 0) => dp[i][j] = left_val + up_val,
                    (1, 0, 0) => dp[i][j] = up_val,
                    (0, 1, 0) => dp[i][j] = left_val,
                    _ => {}
                }
            }
        }
        dp[m - 1][n - 1]
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::unique_paths_with_obstacles(vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 0]
        ])
    );
    println!(
        "{:#?}",
        Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]])
    );
    println!(
        "{:#?}",
        Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 1]])
    );
    println!(
        "{:#?}",
        Solution::unique_paths_with_obstacles(vec![vec![1]])
    );
    println!(
        "{:#?}",
        Solution::unique_paths_with_obstacles(vec![vec![1, 0]])
    );
}
