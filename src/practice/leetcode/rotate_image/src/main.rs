// https://leetcode.com/problems/rotate-image/

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
        for i in 0..n {
            matrix[i].reverse();
        }
    }
}

fn main() {
    println!("Hello, world!");
}
