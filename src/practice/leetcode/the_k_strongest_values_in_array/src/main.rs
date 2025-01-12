// https://leetcode.com/problems/the-k-strongest-values-in-an-array/

struct Solution;

impl Solution {
    pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        arr.sort();
        let median = arr[((arr.len() - 1) / 2) as usize];
        arr.sort_by(|a, b| -> Ordering {
            let ka = (a - median).abs();
            let kb = (b - median).abs();
            if ka != kb {
                return ka.cmp(&kb);
            }
            a.cmp(&b)
        });
        arr[(arr.len() as usize - k as usize)..].to_vec()
    }
}

fn main() {
    let result = Solution::get_strongest(vec![1, 2, 3, 4, 5], 2);
    println!("{:?}", result)
}
