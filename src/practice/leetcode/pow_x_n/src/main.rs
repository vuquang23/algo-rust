struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n == 1 {
            return x;
        }
        if n == -1 {
            return 1.0 / x;
        }
        let half = Solution::my_pow(x, n / 2);
        if n % 2 == 0 {
            return half * half;
        }
        let rest = Solution::my_pow(x, n % 2);
        half * half * rest
    }
}

fn main() {
    let result = Solution::my_pow(2.0, 10);
    println!("{result}")
}
