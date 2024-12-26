// https://leetcode.com/problems/target-sum

struct Solution;

impl Solution {
    fn backtrack(
        dp: &mut Vec<Vec<i32>>,
        rem: &mut Vec<Vec<bool>>,
        nums: &Vec<i32>,
        b: i32,
        ith: usize,
        target: i32,
        s: i32,
    ) -> i32 {
        if ith == nums.len() {
            return if target == 0 { 1 } else { 0 };
        }

        if target > s {
            return 0;
        }
        if target + s < 0 {
            return 0;
        }

        let t = (target + b) as usize;
        if rem[ith][t] {
            return dp[ith][t];
        }
        rem[ith][t] = true;

        dp[ith][t] +=
            Solution::backtrack(dp, rem, nums, b, ith + 1, target - nums[ith], s - nums[ith]);
        dp[ith][t] +=
            Solution::backtrack(dp, rem, nums, b, ith + 1, target + nums[ith], s - nums[ith]);

        dp[ith][t]
    }

    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut s = 0 as i32;
        for i in 0..nums.len() {
            s += nums[i];
        }
        let mn = target - s;
        if mn > 0 {
            return 0;
        }
        let mx = target + s;
        if mx < 0 {
            return 0;
        }
        let b = -mn;

        let n = nums.len();
        let cols = 1 + b + mx;

        let rem_row = vec![false; cols as usize];
        let dp_row = vec![0; cols as usize];

        let mut rem: Vec<Vec<bool>> = vec![rem_row; n];
        let mut dp: Vec<Vec<i32>> = vec![dp_row; n];

        Solution::backtrack(&mut dp, &mut rem, &nums, b, 0, target, s)
    }
}

fn main() {
    let result = Solution::find_target_sum_ways(vec![1, 0], 1);
    print!("{result}\n")
}
