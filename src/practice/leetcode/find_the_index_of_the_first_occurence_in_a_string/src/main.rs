// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(z) => z as i32,
            None => -1,
        }
    }
}

fn main() {
    let result = Solution::str_str("sadbutsad".to_string(), "sad".to_string());
    println!("{result}")
}
