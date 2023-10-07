struct Solution;

// m:0 , n:1
//start/
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];
        let mut my_strs = vec![];
        for i in strs {
            let mut zero: usize = 0;
            let mut one: usize = 0;
            for ele in i.chars() {
                if ele == '0' {
                    zero += 1;
                } else {
                    one += 1;
                }
            }
            my_strs.push((zero, one));
        }

        for st in my_strs {
            for zero in (st.0..=m as usize).rev() {
                for one in (st.1..=n as usize).rev() {
                    dp[zero][one] = dp[zero][one].max(dp[zero - st.0][one - st.1] + 1);
                }
            }
            dbg!(&dp);
        }

        // *dp.last().unwrap().last().unwrap()
        dp[m as usize][n as usize]
    }
}
//end/

fn main() {
    println!(
        "{:#?}",
        Solution::find_max_form(
            vec![
                "10".to_owned(),
                "0001".to_owned(),
                "111001".to_owned(),
                "1".to_owned(),
                "0".to_owned()
            ],
            5,
            3
        )
    );
}
