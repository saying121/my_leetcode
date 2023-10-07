//start/
impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort_by_key(|v| std::cmp::Reverse(v.abs()));
        for i in nums.iter_mut() {
            if k <= 0 {
                break;
            }
            if *i < 0 {
                *i *= -1;
                k -= 1;
            }
        }
        if k % 2 == 1 {
            *nums.last_mut().unwrap() *= -1;
        }
        nums.iter().sum()
    }
}
//end/
struct Solution;
fn main() {
    println!(
        "{:#?}",
        Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1)
    );
    println!(
        "{:#?}",
        Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3)
    );
    println!(
        "{:#?}",
        Solution::largest_sum_after_k_negations(vec![-2, 9, 9, 8, 4], 5)
    );
    println!(
        "{:#?}",
        Solution::largest_sum_after_k_negations(vec![8, -7, -3, -9, 1, 9, -6, -9, 3], 8)
    );
    println!(
        "{:#?}",
        Solution::largest_sum_after_k_negations(vec![5, 6, 9, -3, 3], 2)
    );
}
