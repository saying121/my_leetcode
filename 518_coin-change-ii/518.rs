struct Solution;

//start/
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;
        for ele in coins {
            for j in ele as usize..=amount as usize {
                dp[j] += dp[j - ele as usize];
                // println!("{:?}", dp);
            }
        }
        *dp.last().unwrap()
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::change(5, vec![1, 2, 5]));
    println!("{:#?}", Solution::change(3, vec![2]));
    println!("{:#?}", Solution::change(10, vec![10]));
}

