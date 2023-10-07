struct Solution;

//start/
use std::collections::VecDeque;

impl Solution {
    pub fn monotone_increasing_digits(mut n: i32) -> i32 {
        let mut temp = VecDeque::new();
        while n > 0 {
            temp.push_front(n % 10);
            n /= 10;
        }
        let mut flag = temp.len();
        for i in (1..temp.len()).rev() {
            if temp[i] < temp[i - 1] {
                temp[i - 1] -= 1;
                flag = i;
            }
        }
        for i in flag..temp.len() {
            temp[i] = 9;
        }

        let mut res = 0;
        for i in 0..temp.len() {
            res += temp[i] * 10_i32.pow((temp.len() - 1 - i) as u32);
        }
        res
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::monotone_increasing_digits(10));
    println!("{:#?}", Solution::monotone_increasing_digits(1234));
    println!("{:#?}", Solution::monotone_increasing_digits(332));
    println!("{:#?}", Solution::monotone_increasing_digits(308));
}
