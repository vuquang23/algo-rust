// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::max;
        let n = prices.len();
        let mut dp = vec![vec![0i32; 2]; n + 1];
        dp[0][0] = 6000000;
        dp[1][0] = -prices[0];
        for i in 2..(n + 1) {
            dp[i][0] = max(dp[i - 1][0], dp[i - 2][1] - prices[i - 1]);
            dp[i][1] = max(dp[i - 1][1], dp[i - 1][0] + prices[i - 1]);
        }
        dp[n][1]
    }
}

fn main() {
    let result = Solution::max_profit(vec![1, 2, 3, 0, 2]);
    println!("{result}")
}
