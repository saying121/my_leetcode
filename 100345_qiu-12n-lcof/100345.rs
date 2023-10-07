impl Solution {
    pub fn sum_nums(mut n: i32) -> i32 {
        n == 0 || {
            n += Self::sum_nums(n - 1);
            true
        };

        n
    }
}

// struct Solution;
//
// fn main() {
//     println!("{}", Solution::sum_nums(4));
//     println!("{}", Solution::sum_nums(3));
//     println!("{}", Solution::sum_nums(1));
// }
