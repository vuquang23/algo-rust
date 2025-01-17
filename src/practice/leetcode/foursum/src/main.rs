// https://leetcode.com/problems/4sum

struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::{BTreeMap, BTreeSet};

        let mut cnt: BTreeMap<i32, usize> = BTreeMap::new();
        for &v in &nums {
            let z = match cnt.get(&v) {
                Some(&zz) => zz,
                None => 0,
            };
            cnt.insert(v, z + 1);
        }

        let mut exists: BTreeSet<String> = BTreeSet::new();
        let mut result: Vec<Vec<i32>> = vec![];

        let mut add = |a: i32, b: i32, c: i32, d: i32| {
            let mut v = vec![a, b, c, d];
            v.sort();
            let k = format!("{}|{}|{}|{}", v[0], v[1], v[2], v[3]);
            if exists.contains(&k) {
                return;
            }
            exists.insert(k);
            result.push(v);
        };

        for a in 0..nums.len() {
            for b in (a + 1)..nums.len() {
                for c in (b + 1)..nums.len() {
                    let dd = target as i64 - nums[a] as i64 - nums[b] as i64 - nums[c] as i64;
                    if dd < (i32::MIN as i64) || dd > (i32::MAX as i64) {
                        continue;
                    }
                    let d = dd as i32;
                    let mut cnt_d = match cnt.get(&d) {
                        Some(&z) => z,
                        None => 0,
                    };
                    if d == nums[a] {
                        cnt_d -= 1;
                    }
                    if d == nums[b] {
                        cnt_d -= 1;
                    }
                    if d == nums[c] {
                        cnt_d -= 1;
                    }
                    if cnt_d > 0 {
                        add(nums[a], nums[b], nums[c], d);
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let result = Solution::four_sum(
        vec![1000000000, 1000000000, 1000000000, 1000000000],
        -294967296,
    );
    println!("{:?}", result)
}
