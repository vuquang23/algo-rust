// https://leetcode.com/problems/maximum-subarray/

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        let mut result = i32::MIN;
        let mut min_sum = 0;
        let mut sum = 0 as i32;

        for &v in &nums {
            sum += v;
            result = max(result, sum - min_sum);
            min_sum = min(min_sum, sum)
        }
        result
    }
}

fn main() {
    let result = Solution::max_sub_array(vec![-1]);
    println!("{result}")
}
