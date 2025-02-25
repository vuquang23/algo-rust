// https://leetcode.com/problems/reverse-words-in-a-string/

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .rev()
            .fold("".to_string(), |acc, x| {
                let b = x.trim();
                if b.len() == 0 {
                    acc
                } else {
                    acc + " " + b
                }
            })
            .trim()
            .to_string()
    }
}

fn main() {
    let result = Solution::reverse_words("a good   example".to_string());
    println!("{result}")
}
