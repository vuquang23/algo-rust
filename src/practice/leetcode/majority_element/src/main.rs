// https://leetcode.com/problems/majority-element

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |acc, &x| -> (i32, i32) {
                if acc.1 == 0 {
                    (x, 1)
                } else if acc.0 == x {
                    (acc.0, acc.1 + 1)
                } else {
                    (acc.0, acc.1 - 1)
                }
            })
            .0
    }
}

fn main() {
    let result = Solution::majority_element(vec![3, 2, 3]);
    println!("{result}")
}
