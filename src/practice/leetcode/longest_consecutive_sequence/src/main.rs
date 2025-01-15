// https://leetcode.com/problems/longest-consecutive-sequence/

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut visited: HashMap<i32, bool> = HashMap::new();
        let mut mark: HashMap<i32, bool> = HashMap::new();
        for &n in &nums {
            mark.insert(n, true);
        }
        let mut ans = 0;
        for &n in &nums {
            if *visited.get(&n).unwrap_or(&false) {
                continue;
            }
            let mut itr = n;
            let mut nbr = 0;
            while !visited.get(&itr).unwrap_or(&false) && *mark.get(&itr).unwrap_or(&false) {
                nbr += 1;
                visited.insert(itr, true);
                itr += 1;
            }
            cnt.insert(n, nbr);
            if let Some(&next_cnt) = cnt.get(&itr) {
                *cnt.get_mut(&n).unwrap() += next_cnt;
            }
            ans = ans.max(*cnt.get(&n).unwrap());
        }
        ans
    }
}

fn main() {
    let nums = vec![100, 4, 200, 1, 3, 2];
    let result = Solution::longest_consecutive(nums);
    println!("{result}")
}
