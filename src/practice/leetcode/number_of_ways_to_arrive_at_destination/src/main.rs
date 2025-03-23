// https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/

struct Solution;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        if n == 1 {
            return 1;
        }

        let mut g: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
        for edge in &roads {
            let u = edge[0];
            let v = edge[1];
            let w = edge[2];
            g[u as usize].push((v, w));
            g[v as usize].push((u, w));
        }

        const INF: i64 = 1_000_000_000_000_000_000;
        const MOD: i64 = 1_000_000_007;

        let mut dis = vec![INF; n as usize];
        let mut cnt = vec![0i64; n as usize];
        dis[0] = 0;
        cnt[0] = 1;

        let mut heap = BinaryHeap::new();
        heap.push((0, 0));
        while !heap.is_empty() {
            let top = heap.pop().unwrap();
            let l = -top.0;
            let u = top.1;
            if l != dis[u] {
                continue;
            }
            for edge in &g[u] {
                let v = edge.0 as usize;
                let w = edge.1 as i64;
                let op_u = l + w;
                if dis[v] > op_u {
                    dis[v] = op_u;
                    cnt[v] = cnt[u];
                    heap.push((-dis[v], v));
                } else if dis[v] == op_u {
                    cnt[v] = (cnt[v] + cnt[u]) % MOD;
                }
            }
        }

        cnt[(n - 1) as usize] as i32
    }
}

fn main() {
    let result = Solution::count_paths(
        7,
        vec![
            vec![0, 6, 7],
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![1, 3, 3],
            vec![6, 3, 3],
            vec![3, 5, 1],
            vec![6, 5, 1],
            vec![2, 5, 1],
            vec![0, 4, 5],
            vec![4, 6, 2],
        ],
    );
    println!("{result}")
}
