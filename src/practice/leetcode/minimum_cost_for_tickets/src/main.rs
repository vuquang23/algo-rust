// https://leetcode.com/problems/minimum-cost-for-tickets

struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        let mut dp = [0 as i32; 366];
        let mut mark = [false; 366];
        for &d in days.iter() {
            mark[d as usize] = true;
        }

        for d in 1..366 {
            if mark[d] == false {
                dp[d] = dp[d - 1]
            } else {
                dp[d] = dp[d - 1] + costs[0];
                dp[d] = min(dp[d], dp[max(0, d as i32 - 7) as usize] + costs[1]);
                dp[d] = min(dp[d], dp[max(0, d as i32 - 30) as usize] + costs[2]);
            }
        }

        dp[365]
    }
}

fn main() {
    let result = Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]);
    print!("{result}")
}
