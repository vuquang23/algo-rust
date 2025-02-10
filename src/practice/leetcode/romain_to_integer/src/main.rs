// https://leetcode.com/problems/roman-to-integer

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut prev = 0;
        let mut current = 0;

        for c in s.chars() {
            match c {
                'I' => current = 1,
                'V' => current = 5,
                'X' => current = 10,
                'L' => current = 50,
                'C' => current = 100,
                'D' => current = 500,
                'M' => current = 1000,
                _ => {}
            }

            if current > prev {
                result += current - 2 * prev;
            } else {
                result += current;
            }

            prev = current;
        }

        result
    }
}

fn main() {
    let s = "III".to_string();
    println!("{}", Solution::roman_to_int(s));
}
