// https://leetcode.com/problems/sort-colors

struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0 as i32;
        let mut right = nums.len() as i32 - 1;
        let mut i = 0 as i32;

        while i <= right {
            if nums[i as usize] == 0 {
                nums.swap(i as usize, left as usize);
                left += 1;
                i += 1;
            } else if nums[i as usize] == 2 {
                nums.swap(i as usize, right as usize);
                right -= 1;
            } else {
                i += 1;
            }
        }
    }
}

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
    println!("{:?}", nums);
}
