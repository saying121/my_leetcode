struct Solution;

//start/
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut res = 0;
        let mut cover = 0;
        let mut i = 0;
        loop {
            let mut max = cover;
            for i in i..cover + 1 {
                if i + nums[i as usize] > max {
                    max = i + nums[i as usize];
                }
            }
            i = cover + 1;
            cover = max;
            res += 1;
            if cover as usize >= nums.len() - 1 {
                break;
            }
        }
        res
    }
}
//end/
fn main() {
    println!("{:#?}", Solution::jump(vec![2, 3, 1, 1, 4]));
    println!("{:#?}", Solution::jump(vec![2, 3, 0, 1, 4]));
}
