// https://leetcode.com/problems/count-the-number-of-consistent-strings/

struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut cnt = [false; 26];
        for c in allowed.chars() {
            cnt[(u32::from(c) - 97) as usize] = true;
        }
        let mut result: i32 = 0;
        for s in words.iter() {
            let mut all = true;
            for c in s.chars() {
                if !cnt[(u32::from(c) - 97) as usize] {
                    all = false;
                    break;
                }
            }
            if all {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let result = Solution::count_consistent_strings(
        "ab".to_string(),
        vec![
            "ad".to_string(),
            "bd".to_string(),
            "aaab".to_string(),
            "baa".to_string(),
            "badab".to_string(),
        ],
    );
    print!("{result}")
}
