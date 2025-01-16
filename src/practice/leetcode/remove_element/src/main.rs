// https://leetcode.com/problems/remove-element/

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut last_idx = nums.len() - 1;
        let mut itr = 0;
        while itr < last_idx {
            if nums[itr] == val {
                nums[itr] = nums[last_idx];
                last_idx -= 1;
            } else {
                itr += 1;
            }
        }
        if itr == last_idx && nums[itr] == val {
            last_idx -= 1
        }
        last_idx as i32 + 1
    }
}

fn main() {
    let result = Solution::remove_element(&mut vec![3, 2, 2, 3], 3);
    println!("{result}")
}
