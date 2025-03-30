// https://leetcode.com/problems/string-to-integer-atoi/

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result: i64 = 0;

        let mut sign: i64 = 1;
        let mut got_first = false;

        for c in s.chars() {
            if got_first {
                if c >= '0' && c <= '9' {
                    result = result * 10 + sign * c.to_digit(10).unwrap() as i64;
                    if sign == 1 && result >= i32::MAX as i64 {
                        return i32::MAX;
                    }
                    if sign == -1 && result <= i32::MIN as i64 {
                        return i32::MIN;
                    }
                } else {
                    break;
                }
            } else {
                if c == '+' || c == '-' {
                    got_first = true;
                    if c == '-' {
                        sign = -1;
                    }
                    continue;
                }
                if c >= '0' && c <= '9' {
                    got_first = true;
                    result = c.to_digit(10).unwrap() as i64;
                    continue;
                }
                if c != ' ' {
                    break;
                }
            }
        }

        result as i32
    }
}

fn main() {
    let result = Solution::my_atoi("-6147483648".to_string());
    println!("{result}")
}
