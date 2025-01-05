// https://leetcode.com/problems/most-beautiful-item-for-each-query/

struct Solution;

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        use std::cmp::{max, Ordering};
        struct Data {
            t: u8,
            k: i32,
            v: i32,
        }

        let mut result: Vec<i32> = Vec::with_capacity(queries.len());
        result.resize(queries.len(), 0);
        let mut all: Vec<Data> = Vec::with_capacity(items.len() + queries.len());
        for (_, item) in items.iter().enumerate() {
            all.push(Data {
                t: 0,
                k: item[0],
                v: item[1],
            });
        }
        for (i, q) in queries.iter().enumerate() {
            all.push(Data {
                t: 1,
                k: *q,
                v: i as i32,
            });
        }

        all.sort_by(|a, b| -> Ordering { a.k.cmp(&b.k) });

        let mut mx = 0;
        for d in all.iter() {
            if d.t == 0 {
                mx = max(mx, d.v);
            } else {
                result[d.v as usize] = mx
            }
        }

        result
    }
}

fn main() {
    let result = Solution::maximum_beauty(vec![vec![10, 1000]], vec![5]);
    println!("{:?}", result);
}
