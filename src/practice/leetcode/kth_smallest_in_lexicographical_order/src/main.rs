struct Solution;

impl Solution {
    fn cal_steps(n: i32, n1: i32, n2: i32) -> i32 {
        let mut steps = 0;
        let mut n1 = n1 as i64;
        let mut n2 = n2 as i64;
        while n1 <= n as i64 {
            steps += i64::min(n as i64 + 1, n2) - n1;
            n1 *= 10;
            n2 *= 10;
        }
        steps as i32
    }

    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut k = k;
        let mut cur = 1;
        k -= 1;
        while k > 0 {
            let step = Self::cal_steps(n, cur, cur + 1);
            if step <= k {
                cur += 1;
                k -= step;
            } else {
                cur *= 10;
                k -= 1;
            }
        }
        cur as i32
    }
}

fn main() {
    let n = 13;
    let k = 2;
    let result = Solution::find_kth_number(n, k);
    println!("Result: {}", result);
}
