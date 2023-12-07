struct Solution;

//start/
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hmap = HashMap::new();

        for (i, &val) in nums.iter().enumerate() {
            if let Some(&j) = hmap.get(&(target - nums[i])) {
                return vec![j, i as i32];
            }
            hmap.insert(val, i as i32);
        }

        vec![]
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:#?}", Solution::two_sum(vec![3, 2, 4], 6));
    println!("{:#?}", Solution::two_sum(vec![3, 3], 6));
}
