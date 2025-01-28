struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        use std::cmp::max;
        let mut mx: i32 = 0;
        let mut steps: i32 = 0;
        let mut current_end = 0;
        for i in 0..nums.len() - 1 {
            mx = max(mx, i as i32 + nums[i]);
            if i == current_end {
                steps += 1;
                current_end = mx as usize;
            }
        }
        return steps;
    }
}

fn main() {
    let result = Solution::jump(vec![2, 3, 1, 1, 4]);
    println!("{result}")
}
