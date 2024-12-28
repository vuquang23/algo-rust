// https://leetcode.com/problems/integer-replacement

struct Solution;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        use std::cmp::min;
        use std::collections::BTreeMap;
        use std::collections::BTreeSet;

        if n == 1 {
            return 0;
        }
        let mut pw2 = [0 as u64; 32];
        pw2[0] = 1;
        for i in 1..32 {
            pw2[i] = pw2[i - 1] * 2;
        }
        let mut rem: BTreeSet<i32> = BTreeSet::new();
        let mut dp: BTreeMap<i32, i32> = BTreeMap::new();
        fn backtrack(
            rem: &mut BTreeSet<i32>,
            dp: &mut BTreeMap<i32, i32>,
            pw2: &[u64; 32],
            value: i32,
        ) -> i32 {
            match pw2.iter().position(|&z| z == (value as u64)) {
                Some(idx) => return idx as i32,
                _ => {}
            };
            let cache = rem.get(&value);
            if let Some(key) = cache {
                let result = dp.get(key);
                match result {
                    Some(&r) => return r,
                    _ => return -1,
                }
            }
            rem.insert(value);
            let ret: i32;
            if value % 2 == 0 {
                let rr = backtrack(rem, dp, pw2, value >> 1);
                ret = rr + 1;
                dp.insert(value, ret);
            } else {
                let add1 = backtrack(rem, dp, pw2, value + 1);
                let minus1 = backtrack(rem, dp, pw2, value - 1);
                ret = min(add1, minus1) + 1;
                dp.insert(value, ret);
            }
            ret
        }
        backtrack(&mut rem, &mut dp, &pw2, n)
    }
}

fn main() {
    let result = Solution::integer_replacement(8);
    print!("{result}\n");
}
