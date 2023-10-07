//start/
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_sum = 0;
        let mut cur_sum = 0;
        let mut res = 0;
        for i in 0..gas.len() {
            total_sum += gas[i] - cost[i];
            cur_sum += gas[i] - cost[i];
            if cur_sum < 0 {
                res = i + 1;
                cur_sum = 0;
            }
        }
        if total_sum < 0 {
            return -1;
        }
        res as i32
    }
}
//end/
struct Solution;
fn main() {
    println!(
        "{:#?}",
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
    );
    println!(
        "{:#?}",
        Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3])
    );
    println!(
        "{:#?}",
        Solution::can_complete_circuit(vec![0, 0, 0], vec![0, 0, 0])
    );
}
