// https://leetcode.com/problems/count-primes/

struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut d: Vec<bool> = vec![];
        d.resize(n as usize + 1, false);

        for i in 2..n as usize {
            let mut j = i * i;
            while j <= n as usize {
                d[j] = true;
                j += i;
            }
        }

        let mut result = 0;
        for i in 2..n as usize {
            result += if d[i] == false { 1 } else { 0 }
        }

        result
    }
}

fn main() {
    let result = Solution::count_primes(10);
    println!("{result}")
}
