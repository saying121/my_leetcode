struct Solution;

//start/
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let len = nums.len();
        let mut new_nums = nums;
        new_nums.sort();

        for a in 0..len - 2 {
            if new_nums[a] > 0 {
                break;
            }
            let mut b = a + 1;
            let mut c = len - 1;

            if a > 0 && (new_nums[a] == new_nums[a - 1]) {
                continue;
            }

            while b < c {
                let sum = new_nums[a] + new_nums[b] + new_nums[c];

                if sum > 0 {
                    c -= 1;
                } else if sum < 0 {
                    b += 1;
                } else {
                    result.push(vec![new_nums[a], new_nums[b], new_nums[c]]);

                    while b < c && new_nums[b] == new_nums[b + 1] {
                        b += 1;
                    }
                    while b < c && new_nums[c] == new_nums[c - 1] {
                        c -= 1;
                    }

                    b += 1;
                    c -= 1;
                }
            }
        }

        result
    }
}
//end/

fn main() {
    println!("{:?}", Solution::three_sum1(vec![-1, 0, 1, 2, -1, -4]));
    println!("{:?}", Solution::three_sum(vec![0, 1, 1]));
    println!("{:?}", Solution::three_sum(vec![0, 0, 0, 0]));
    println!("{:?}", Solution::three_sum1(vec![0, 0, 0]));
}
use std::cmp::Ordering;
impl Solution {
    pub fn three_sum1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        nums.sort();
        let len = nums.len();
        for i in 0..len {
            if nums[i] > 0 {
                return result;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut left, mut right) = (i + 1, len - 1);
            while left < right {
                match (nums[i] + nums[left] + nums[right]).cmp(&0) {
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    }
                    Ordering::Greater => right -= 1,
                    Ordering::Less => left += 1,
                }
            }
        }
        result
    }
}
