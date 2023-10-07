struct Solution;

//start/
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tail = Vec::with_capacity(nums.len());
        tail.push(nums[0]);
        for &i in nums.iter().skip(1) {
            if *tail.last().unwrap() < i {
                tail.push(i);
            } else {
                // let (mut left, mut right) = (0, tail.len() - 1);
                //
                // while left < right {
                //     let mid = (left + right) / 2;
                //     if tail[mid] >= i {
                //         right = mid;
                //     } else {
                //         left = mid + 1;
                //     }
                // }
                // tail[left] = i;

                let index = tail.partition_point(|x| x < &i);
                tail[index] = i;
            }
            // println!("{:?}", tail);
        }
        tail.len() as i32
    }
    // pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    //     let mut res = 1;
    //     let mut dp = vec![1; nums.len()];
    //     for i in 1..nums.len() {
    //         for j in 0..i {
    //             if nums[j] < nums[i] {
    //                 dp[i] = dp[i].max(dp[j] + 1);
    //             }
    //         }
    //         res = res.max(dp[i]);
    //     }
    //     res
    // }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18])
    );
    println!("{:#?}", Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));
    println!("{:#?}", Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]));
    // println!("{:#?}", Solution::length_of_lis(vec![7, 6, 5, 4, 3, 2, 1]));
    println!("{:#?}", Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));

    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    dbg!(s.binary_search(&1));
    assert_eq!(s.binary_search(&13), Ok(9));

    let tail = [1, 1, 3, 3, 3, 4];
    // let tail = [1];
    let target = 3;
    let (mut left, mut right) = (0, tail.len() - 1);

    while left <= right {
        let mid = left + (right - left) / 2;

        if tail[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    dbg!(&left);
    dbg!(&right);
}
