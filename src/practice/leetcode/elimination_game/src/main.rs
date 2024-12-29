// https://leetcode.com/problems/elimination-game

struct Solution;

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        return 2 * (n / 2 + 1 - Solution::last_remaining(n / 2));
    }
}

fn main() {
    let result = Solution::last_remaining(9);
    print!("{result}")
}
