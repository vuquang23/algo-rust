// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k

struct Solution;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::cmp::max;
        use std::collections::BTreeSet;
        use std::i32;
        let row = matrix.len();
        let col = matrix[0].len();
        let mut res = i32::MIN;
        for i in 0..row {
            let mut sum = vec![0 as i32; col];
            for j in i..row {
                for c in 0..col {
                    sum[c] += matrix[j][c];
                }
                let mut btree_set: BTreeSet<i32> = BTreeSet::new();
                btree_set.insert(0);
                let mut run = 0;
                for &sum in &sum {
                    run += sum;
                    if let Some(&value) = btree_set.range((run - k)..).next() {
                        res = max(res, run - value);
                    }
                    btree_set.insert(run);
                }
            }
        }
        res
    }
}

fn main() {
    let result = Solution::max_sum_submatrix(
        vec![vec![5, -4, -3, 4], vec![-3, -4, 4, 5], vec![5, 1, 5, -4]],
        -2,
    );
    println!("{result}")
}
