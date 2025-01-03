// https://leetcode.com/problems/sum-of-number-and-its-reverse/

struct Solution;

impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        if num == 0 {
            return true;
        }
        let reverse_number = |mut n: i32| -> i32 {
            let mut ret = 0;
            while n > 0 {
                ret = ret * 10 + (n % 10);
                n /= 10;
            }
            ret
        };
        for i in 1..num {
            if i + reverse_number(i) == num {
                return true;
            }
        }
        false
    }
}

fn main() {
    let result = Solution::sum_of_number_and_reverse(443);
    print!("{result}")
}
