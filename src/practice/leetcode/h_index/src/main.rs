// https://leetcode.com/problems/h-index/

struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut cnt: Vec<usize> = vec![];
        cnt.resize(1001, 0);
        for &z in &citations {
            cnt[z as usize] += 1;
        }
        for ii in 1..1001 {
            let i: usize = 1000 - ii;
            cnt[i] += cnt[i + 1];
            if cnt[i] >= i {
                return i as i32;
            }
        }
        0 as i32
    }
}

fn main() {
    let result = Solution::h_index(vec![3, 0, 6, 1, 5]);
    println!("{result}")
}
