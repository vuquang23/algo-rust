// https://leetcode.com/problems/construct-smallest-number-from-di-string/

struct Solution;

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut ans = "1".to_string();
        let mut temp = String::new();

        for (i, &c) in pattern.as_bytes().iter().enumerate() {
            if c == b'I' {
                ans.push_str(&temp);
                ans.push((b'2' + i as u8) as char);
                temp.clear();
            } else {
                temp = ans.pop().unwrap().to_string() + &temp;
                ans.push((b'2' + i as u8) as char);
            }
        }
        ans + &temp
    }
}

fn main() {
    let result = Solution::smallest_number("IIIDIDDD".to_string());
    println!("{result}")
}
