struct Solution;

//start/
impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived
            .into_iter()
            .reduce(|a, b| a ^ b)
            .unwrap()
            == 0
    }
}
//end/

fn main() {
    println!("{:#?}", Solution::does_valid_array_exist(vec![1, 1, 0]));
}

