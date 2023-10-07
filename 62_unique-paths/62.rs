struct Solution;

//start/
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let all = m + n - 2;
        let up = m - 1;

        let mut res = 1;
        let mut numerator: u128 = all as u128;
        let mut denominator: u128 = up as u128;

        for _ in 1..=up {
            res *= numerator;
            while denominator > 0 && res % denominator == 0 {
                res /= denominator;
                denominator -= 1;
            }

            numerator -= 1;
        }

        res as i32
    }

    // pub fn unique_paths(m: i32, n: i32) -> i32 {
    //     let di = m + n - 2;
    //     let up = m - 1;
    //     let mut numerator: i32 = (di - up + 1..=di).product();
    //     let mut denominator: i32 = (1..=up).product();
    //     numerator / denominator
    // }

    // pub fn unique_paths(m: i32, n: i32) -> i32 {
    //     let mut dp = vec![vec![1; n as usize]; m as usize];
    //     for i in 1..m as usize {
    //         for j in 1..n as usize {
    //             dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
    //         }
    //     }
    //     dp[m as usize - 1][n as usize - 1]
    // }
}
//end/

fn main() {
    println!("{:#?}", Solution::unique_paths(3, 7));
    println!("{:#?}", Solution::unique_paths(7, 3));
    println!("{:#?}", Solution::unique_paths(3, 2));
    println!("{:#?}", Solution::unique_paths(1, 1));
    println!("{:#?}", Solution::unique_paths(2, 2));
    println!("{:#?}", Solution::unique_paths(23, 12));
    println!("{:#?}", Solution::unique_paths(4, 100));
    println!("{:#?}", Solution::unique_paths(99, 99));
}
