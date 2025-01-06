// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/

struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut ans = 0;
        for i in 0..25 {
            let mut cnt = 0;
            for &num in candidates.iter() {
                if (num & (1 << i)) != 0 {
                    cnt += 1;
                }
            }
            ans = max(ans, cnt);
        }
        ans
    }
}

fn main() {
    let result = Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14]);
    println!("{result}");
}
