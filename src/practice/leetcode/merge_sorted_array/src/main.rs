// https://leetcode.com/problems/merge-sorted-array

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in m..(m + n) {
            nums1[i as usize] = nums2[i as usize - m as usize]
        }
        nums1.sort();
    }
}

fn main() {
    Solution::merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3);
}
