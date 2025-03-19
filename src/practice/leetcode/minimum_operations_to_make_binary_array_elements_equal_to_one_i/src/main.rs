// https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/

struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                if i + 2 >= nums.len() {
                    return -1;
                }
                result += 1;
                // nums[i] = 1;
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
            }
        }
        result
    }
}

fn main() {
    let result = Solution::min_operations(vec![0, 1, 1, 1, 0, 0]);
    println!("{result}")
}
