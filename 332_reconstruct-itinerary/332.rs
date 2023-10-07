//start/
impl Solution {
    pub fn find_itinerary(mut tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut res = vec!["JFK".to_string()];
        let mut used = vec![false; tickets.len()];
        tickets.sort();

        Self::back_tracking(&tickets, &mut res, &mut used);

        res
    }
    fn back_tracking(
        tickets: &Vec<Vec<String>>,
        res: &mut Vec<String>,
        used: &mut Vec<bool>,
    ) -> bool {
        if res.len() == tickets.len() + 1 {
            return true;
        }
        for i in 0..tickets.len() {
            if used[i] {
                continue;
            }
            let temp = tickets.get(i).unwrap();
            if &temp[0] != res.last().unwrap() {
                continue;
            }
            res.push(temp[1].to_string());
            used[i] = true;
            if Self::back_tracking(tickets, res, used) {
                return true;
            };
            used[i] = false;
            res.pop();
        }
        false
    }
}
//end/
struct Solution;
fn main() {
    println!(
        "{:#?}",
        Solution::find_itinerary(vec![
            vec!["MUC".to_string(), "LHR".to_string()],
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["SFO".to_string(), "SJC".to_string()],
            vec!["LHR".to_string(), "SFO".to_string()]
        ])
    );
    println!(
        "{:#?}",
        Solution::find_itinerary(vec![
            vec!["JFK".to_string(), "SFO".to_string()],
            vec!["JFK".to_string(), "ATL".to_string()],
            vec!["SFO".to_string(), "ATL".to_string()],
            vec!["ATL".to_string(), "JFK".to_string()],
            vec!["ATL".to_string(), "SFO".to_string()]
        ])
    );
}
