// https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum

struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        use std::cmp::min;
        let inf = 100001;
        let mut f: Vec<i32> = Vec::with_capacity(arr.len());
        f.resize(arr.len(), inf);

        let mut idx = 0;
        let mut current_sum = 0;
        for i in 0..arr.len() {
            if i == 0 {
                current_sum = arr[i];
            } else {
                current_sum += arr[i];
                while idx < i && current_sum > target {
                    current_sum -= arr[idx];
                    idx += 1;
                }
            }
            if i > 0 {
                f[i] = f[i - 1];
            }
            if current_sum == target {
                if i == 0 {
                    f[i] = 1;
                } else {
                    f[i] = min(f[i], i as i32 - idx as i32 + 1);
                }
            }
        }

        let mut result = inf;
        idx = arr.len() - 1;
        current_sum = 0;
        let mut best_right = inf;
        for ii in 0..arr.len() - 1 {
            let i = arr.len() - 1 - ii;
            if i == arr.len() - 1 {
                current_sum = arr[i];
            } else {
                current_sum += arr[i];
                while idx > i && current_sum > target {
                    current_sum -= arr[idx];
                    idx -= 1;
                }
            }
            if current_sum == target {
                best_right = min(best_right, idx as i32 - i as i32 + 1);
            }
            result = min(result, best_right + f[i - 1]);
        }
        if result != inf {
            result
        } else {
            -1
        }
    }
}

fn main() {
    let result = Solution::min_sum_of_lengths(vec![1, 2, 2, 3, 2, 6, 7, 2, 1, 4, 8], 5);
    println!("{result}")
}
