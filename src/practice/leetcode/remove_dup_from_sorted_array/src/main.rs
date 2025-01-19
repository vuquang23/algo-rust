// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        use std::collections::BTreeSet;
        let mut set: BTreeSet<i32> = BTreeSet::new();
        let mut idx = 0;
        for i in 0..nums.len() {
            let ok = set.contains(&nums[i]);
            if ok {
                continue;
            }
            set.insert(nums[i]);
            nums[idx] = nums[i];
            idx += 1;
        }
        idx as i32
    }
}

fn main() {
    let result = Solution::remove_duplicates(&mut vec![1, 1, 2]);
    println!("{result}")
}
