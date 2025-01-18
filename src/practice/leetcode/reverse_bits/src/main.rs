// https://leetcode.com/problems/reverse-bits/

struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut result: u32 = 0;
        for i in 0..32 {
            if (x >> i) & 1 == 1 {
                result += 1 << (31 - i);
            }
        }
        result
    }
}

fn main() {}
