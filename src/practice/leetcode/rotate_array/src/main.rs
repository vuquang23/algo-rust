// https://leetcode.com/problems/rotate-array

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let z = k as usize % nums.len();
        nums.rotate_right(z);
    }
}

fn main() {
    let mut data = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut data, 3);
    println!("{:?}", data);
}
