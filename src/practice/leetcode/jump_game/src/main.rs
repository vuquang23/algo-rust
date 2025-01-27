// https://leetcode.com/problems/jump-game/

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 0 {
            return true;
        }
        use std::cmp::max;
        let mut mx = 0 as i32;
        for i in 0..nums.len() {
            if mx < i as i32 {
                return false;
            }
            mx = max(mx, i as i32 + nums[i]);
        }
        mx >= nums.len() as i32 - 1
    }
}

fn main() {
    let result = Solution::can_jump(vec![2, 3, 1, 1, 4]);
    println!("{result}")
}
