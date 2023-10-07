struct Solution;

//start/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            _ => {
                let mut first = 1;
                let mut second = 2;
                let mut res = 0;
                for _ in 2..n {
                    res = first + second;
                    first = second;
                    second = res;
                }
                res
            }
        }
    }
    // pub fn climb_stairs(n: i32) -> i32 {
    //     let mut path = vec![1; n as usize + 1];
    //     for i in 2..n as usize + 1 {
    //         path[i] = path[i - 1] + path[i - 2];
    //     }
    //     path[n as usize]
    // }
}
//end/

fn main() {
    println!("{:#?}", Solution::climb_stairs(2));
    println!("{:#?}", Solution::climb_stairs(3));
    println!("{:#?}", Solution::climb_stairs(4));
}
