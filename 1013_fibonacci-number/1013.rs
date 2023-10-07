struct Solution;

//start/
impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut first = 0;
                let mut second = 1;
                let mut res = 0;
                for _ in 1..n {
                    res = first + second;
                    first = second;
                    second = res;
                }
                res
            }
        }
    }
    // pub fn fib(n: i32) -> i32 {
    //     let mut dp = vec![0; n as usize + 1];
    //     if n >= 1 {
    //         dp[1] = 1;
    //     }
    //     for i in 2..n as usize + 1 {
    //         dp[i] = dp[i - 1] + dp[i - 2];
    //     }
    //     dp[n as usize]
    // }
}
//end/

fn main() {
    println!("{:#?}", Solution::fib(0));
    println!("{:#?}", Solution::fib(1));
    println!("{:#?}", Solution::fib(2));
    println!("{:#?}", Solution::fib(3));
    println!("{:#?}", Solution::fib(4));
}
