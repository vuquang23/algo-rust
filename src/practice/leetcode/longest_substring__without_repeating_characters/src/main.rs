// https://leetcode.com/problems/longest-substring-without-repeating-characters/

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::{max, min};
        use std::collections::HashMap;
        let mut last: HashMap<char, usize> = HashMap::new();
        let mut result = 0 as i32;
        let mut current = 0 as i32;
        for (i, c) in s.chars().enumerate() {
            current += 1;
            match last.get(&c) {
                Some(j) => current = min(current, (i - j) as i32),
                _ => {}
            };
            last.insert(c, i);
            result = max(result, current);
        }
        result
    }
}

fn main() {
    let result = Solution::length_of_longest_substring("abcabcbb".to_string());
    println!("{result}")
}
