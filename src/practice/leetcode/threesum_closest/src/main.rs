// https://leetcode.com/problems/3sum-closest/

struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 1_000_000_000;
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if (sum - target).abs() < (result - target).abs() {
                    result = sum;
                }
                if sum > target {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        result
    }
}

fn main() {
    let result = Solution::three_sum_closest(vec![-1, 2, 1, -4], 1);
    println!("{result}")
}
