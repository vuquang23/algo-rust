// https://leetcode.com/problems/most-beautiful-item-for-each-query/

struct Solution;

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        use std::cmp::{max, Ordering};
        struct Data {
            t: u8,
            idx: usize,
        }

        let mut result: Vec<i32> = Vec::with_capacity(queries.len());
        result.resize(queries.len(), 0);
        let mut all: Vec<Data> = Vec::with_capacity(items.len() + queries.len());
        for i in 0..items.len() {
            all.push(Data { t: 0, idx: i });
        }
        for i in 0..queries.len() {
            all.push(Data { t: 1, idx: i });
        }

        all.sort_by(|a, b| -> Ordering {
            let price_a = if a.t == 0 {
                items[a.idx][0]
            } else {
                queries[a.idx]
            };
            let price_b = if b.t == 0 {
                items[b.idx][0]
            } else {
                queries[b.idx]
            };
            price_a.cmp(&price_b)
        });

        let mut mx = 0;
        for d in all.iter() {
            if d.t == 0 {
                mx = max(mx, items[d.idx][1]);
            } else {
                result[d.idx] = mx
            }
        }

        result
    }
}

fn main() {
    let result = Solution::maximum_beauty(vec![vec![10, 1000]], vec![5]);
    println!("{:?}", result);
}
