// https://leetcode.com/problems/network-delay-time/

struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        use std::cmp::{max, min};

        const INF: i32 = 1_000_000_000;
        let mut dp = vec![vec![INF; n as usize]; n as usize];
        // We don't need edges array since we're not using it

        for i in 0..n {
            dp[i as usize][i as usize] = 0;
        }
        for t in &times {
            let u = t[0] as usize - 1;
            let v = t[1] as usize - 1;
            let w = t[2];
            dp[u][v] = min(dp[u][v], w);
        }

        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
                }
            }
        }

        let k_idx = k as usize - 1;
        let mut max_time = 0;

        for i in 0..n as usize {
            if dp[k_idx][i] == INF {
                return -1; // If any node is unreachable, return -1
            }
            max_time = max(max_time, dp[k_idx][i]);
        }

        max_time
    }
}

fn main() {
    let res = Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2);
    println!("{res}")
}
