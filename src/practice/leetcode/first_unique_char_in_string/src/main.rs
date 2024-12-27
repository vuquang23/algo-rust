// https://leetcode.com/problems/first-unique-character-in-a-string

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut cnt = [0 as usize; 26];
        for c in s.chars() {
            cnt[(u32::from(c) - 97) as usize] += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if cnt[(u32::from(c) - 97) as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {
    let result = Solution::first_uniq_char("leetcode".to_string());
    print!("{result}")
}
