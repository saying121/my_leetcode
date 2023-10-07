struct Solution;

//start/
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        if (sum - target) % 2 == 1 || target.abs() > sum {
            return 0;
        }
        let cap = ((sum - target) / 2) as usize;
        let mut dp = vec![0; cap + 1];
        dp[0] = 1;

        for num in nums {
            for j in (num as usize..=cap).rev() {
                dp[j] += dp[j - num as usize];
                // println!("{:?}", dp);
            }
        }

        dp[cap]
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3)
    );
    println!("{:#?}", Solution::find_target_sum_ways(vec![1, 1, 1], 1));
    println!("{:#?}", Solution::find_target_sum_ways(vec![1], -1));
    println!("{:#?}", Solution::find_target_sum_ways(vec![1, 0], 1));
    println!("{:#?}", Solution::find_target_sum_ways(vec![1, 0, 0], 1));
    println!("{:#?}", Solution::find_target_sum_ways(vec![1, 0, 0, 0], 1));
    println!("{:#?}", Solution::find_target_sum_ways(vec![1, 1], 0));
    println!("{:#?}", Solution::find_target_sum_ways(vec![1, 2], 3));
    println!("{:#?}", Solution::find_target_sum_ways(vec![3, 2], 1));
}
