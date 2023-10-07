//start/
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut five, mut ten) = (0, 0);
        for i in bills {
            if i == 5 {
                five += 1;
            }
            if i == 10 {
                ten += 1;
                if five >= 1 {
                    five -= 1;
                } else {
                    return false;
                }
            }
            if i == 20 {
                if ten >= 1 && five >= 1 {
                    ten -= 1;
                    five -= 1;
                } else if five >= 3 {
                    five -= 3;
                } else {
                    return false;
                }
            }
        }
        true
    }
}
//end/
struct Solution;
fn main() {
    println!("{:#?}", Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    println!("{:#?}", Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
}

