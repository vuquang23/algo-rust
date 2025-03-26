// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/description/

struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut divs: Vec<i32> = vec![];
        let mut m = -1;
        for row in &grid {
            for &value in row {
                let mm = value % x;
                if m == -1 {
                    m = mm;
                } else if mm != m {
                    return -1;
                }
                divs.push(value / x);
            }
        }
        divs.sort();
        let k = divs[divs.len() / 2];
        let mut result = 0;
        for &d in &divs {
            if k >= d {
                result += k - d;
            } else {
                result += d - k;
            }
        }
        result
    }
}

fn main() {
    let result = Solution::min_operations(vec![vec![4], vec![1]], 2);
    println!("{result}")
}
