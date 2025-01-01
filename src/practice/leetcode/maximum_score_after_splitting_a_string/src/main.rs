// https://leetcode.com/problems/maximum-score-after-splitting-a-string

struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        use std::cmp::max;
        let mut cnt: Vec<i32> = vec![];
        for (i, c) in s.chars().enumerate() {
            let k = (if c == '1' { 0 } else { 1 }) + (if i > 0 { cnt[i - 1] } else { 0 });
            cnt.push(k);
        }
        let mut result = 0;
        let l = s.len() as i32;
        for i in 0..(l - 1) {
            let k = l - 1 - i - cnt[l as usize - 1] + 2 * cnt[i as usize];
            result = max(result, k.abs())
        }
        result
    }
}

fn main() {
    let result = Solution::max_score("1111".to_string());
    print!("{result}")
}
