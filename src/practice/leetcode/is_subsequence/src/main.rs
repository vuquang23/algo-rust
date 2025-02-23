// https://leetcode.com/problems/is-subsequence/

pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars();
        let mut t_chars = t.chars();
        let mut s_char = s_chars.next();
        let mut t_char = t_chars.next();
        while s_char.is_some() && t_char.is_some() {
            if s_char.unwrap() == t_char.unwrap() {
                s_char = s_chars.next();
            }
            t_char = t_chars.next();
        }
        s_char.is_none()
    }
}

fn main() {
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence(s, t);
    println!("{}", result);
}
