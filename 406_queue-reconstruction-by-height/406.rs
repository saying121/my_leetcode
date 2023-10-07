struct Solution;

//start/
use std::collections::LinkedList;

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut queue = LinkedList::new();
        // let mut queue = vec![];
        people.sort_by(|a, b| {
            if a[0] == b[0] {
                return a[1].cmp(&b[1]);
            }
            b[0].cmp(&a[0])
        });
        dbg!(&people);
        queue.push_back(people[0].clone());
        for v in people.iter().skip(1) {
            if queue.len() > v[1] as usize {
                let mut back_link = queue.split_off(v[1] as usize);
                queue.push_back(v.clone());
                queue.append(&mut back_link);
            } else {
                queue.push_back(v.clone());
            }
        }
        queue.into_iter().collect()
    }
}
//end/

fn main() {
    println!(
        "{:?}",
        Solution::reconstruct_queue(vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2]
        ])
    );
    println!(
        "{:?}",
        Solution::reconstruct_queue(vec![
            vec![9, 0],
            vec![7, 0],
            vec![1, 9],
            vec![3, 0],
            vec![2, 7],
            vec![5, 3],
            vec![6, 0],
            vec![3, 4],
            vec![6, 2],
            vec![5, 2]
        ])
    );
}
