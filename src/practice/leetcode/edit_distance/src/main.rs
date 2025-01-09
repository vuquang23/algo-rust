// https://leetcode.com/problems/edit-distance/

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        use std::cmp::min;
        if word1.len() == 0 {
            return if word2.len() == 0 {
                0
            } else {
                word2.len() as i32
            };
        }
        if word2.len() == 0 {
            return word1.len() as i32;
        }
        let inf = 1000 as i32;
        let mut dp = vec![vec![inf; word2.len() + 1]; word1.len() + 1];
        dp[0][0] = 0;
        for i in 0..word1.len() {
            let k = i + 1;
            dp[k][0] = k as i32;
        }
        for j in 0..word2.len() {
            let k = j + 1;
            dp[0][k] = k as i32;
        }
        for (wi, w1) in word1.chars().enumerate() {
            let i = wi + 1;
            for (wj, w2) in word2.chars().enumerate() {
                let j = wj + 1;
                if w1 == w2 {
                    dp[i][j] = min(dp[i][j], dp[wi][wj]); // do nothing
                } else {
                    dp[i][j] = min(dp[i][j], dp[wi][wj] + 1); // replace
                }
                dp[i][j] = min(dp[i][j], dp[i][wj] + 1); // insert
                dp[i][j] = min(dp[i][j], dp[wi][j] + 1); // delete
            }
        }
        dp[word1.len()][word2.len()]
    }
}

fn main() {
    let result = Solution::min_distance("sea".to_string(), "eat".to_string());
    println!("{result}")
}
