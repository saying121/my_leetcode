struct Solution;

//start/
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums2.len()];
        let mut res = 0;
        for i in 0..nums1.len() {
            for j in (0..nums2.len()).rev() {
                if nums1[i] == nums2[j] {
                    dp[j] = if j == 0 { 1 } else { dp[j - 1] + 1 };
                    res = res.max(dp[j]);
                } else {
                    dp[j] = 0;
                }
                // println!("{:?}", dp);
            }
        }
        res
    }
    pub fn find_length1(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut res = 0;
        for i in 1..=nums1.len() {
            for j in 1..=nums2.len() {
                if nums2[j - 1] == nums1[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                }
                for ele in &dp {
                    println!("{:?}", ele);
                }
                println!();
                res = res.max(dp[i][j]);
            }
        }
        res
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7])
    );
    println!(
        "{:#?}",
        Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0])
    );
    println!(
        "{:#?}",
        Solution::find_length(vec![1, 0, 0, 0, 1], vec![1, 0, 0, 1, 1])
    );
    println!(
        "{:#?}",
        Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7])
    );
}
