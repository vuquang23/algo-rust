// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/

struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let words: Vec<&str> = sentence.split(" ").collect();
        for (i, &w) in words.iter().enumerate() {
            if w.starts_with(&search_word) {
                return i as i32 + 1;
            }
        }
        -1
    }
}

fn main() {
    let result =
        Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string());
    println!("{result}")
}
