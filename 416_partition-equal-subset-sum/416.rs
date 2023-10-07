struct Solution;

//start/
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 1 {
            return false;
        }
        let cap = (sum / 2) as usize;
        let mut dp = vec![0; cap + 1];
        // println!("{:?}", dp);

        for n in nums {
            for j in (n as usize..=cap).rev() {
                dp[j] = dp[j].max(dp[j - n as usize] + n);
                // println!("{:?}", dp);
                if dp[cap] == cap as i32 {
                    return true;
                }
            }
        }

        false
    }
    pub fn can_partition1(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 1 {
            return false;
        }
        let cap = sum as usize / 2;
        let mut dp = vec![vec![0; cap + 1]; nums.len()];
        for i in nums[0] as usize..cap + 1 {
            dp[0][i] = nums[0];
        }
        // prt(&dp);

        for i in 1..nums.len() {
            for j in 1..cap + 1 {
                if (j as i32) < nums[i] {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] =
                        dp[i - 1][j].max(dp[i - 1][j - nums[i] as usize] + nums[i]);
                }
            }
        }
        prt(&dp);

        cap as i32 == *dp.last().unwrap().last().unwrap()
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::can_partition1(vec![1, 5, 11, 5]));
    println!("{:#?}", Solution::can_partition(vec![1, 2, 3, 5]));
    println!("{:#?}", Solution::can_partition(vec![2, 2, 1, 1]));
    println!("{:#?}", Solution::can_partition(vec![1, 2, 5]));
}

fn prt(a: &Vec<Vec<i32>>) {
    for ele in a {
        println!("{:?}", ele);
    }
    println!();
}
