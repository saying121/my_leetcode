//start/
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let mut res = nums[0];
        let mut i = 0;
        while i <= res {
            res = res.max(nums[i as usize] + i);
            if res as usize >= nums.len() - 1 {
                return true;
            }
            i += 1;
        }
        false
    }
}
//end/
struct Solution;
fn main() {
    println!("{:#?}", Solution::can_jump(vec![2, 3, 1, 1, 4]));
    println!("{:#?}", Solution::can_jump(vec![3, 2, 1, 0, 4]));
    println!("{:#?}", Solution::can_jump(vec![1, 2, 3]));
}
