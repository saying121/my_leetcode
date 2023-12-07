struct Solution;

//start/
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 1 + nums1.len()]; 1 + nums2.len()];
        for one in 1..=nums1.len() {
            for two in 1..=nums2.len() {
                if nums1[one - 1] == nums2[two - 1] {
                    dp[two][one] = dp[two - 1][one - 1] + 1;
                } else {
                    dp[two][one] = dp[two][one - 1].max(dp[two - 1][one]);
                }
            }
        }
        *dp.last().unwrap().last().unwrap()
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::max_uncrossed_lines(vec![2, 1], vec![1, 2, 1, 3, 3, 2])
    );
    // println!(
    //     "{:#?}",
    //     Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4])
    // );
    // println!(
    //     "{:#?}",
    //     Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2])
    // );
    // println!(
    //     "{:#?}",
    //     Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1])
    // );
}
