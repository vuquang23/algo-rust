// https://leetcode.com/problems/count-prefix-and-suffix-pairs-i

struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut result = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if words[j].starts_with(&words[i]) && words[j].ends_with(&words[i]) {
                    result += 1;
                }
            }
        }
        result
    }
}

fn main() {
    let result = Solution::count_prefix_suffix_pairs(vec![
        "a".to_string(),
        "aba".to_string(),
        "ababa".to_string(),
        "aa".to_string(),
    ]);
    println!("{result}")
}
