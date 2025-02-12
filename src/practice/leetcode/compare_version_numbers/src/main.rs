// https://leetcode.com/problems/compare-version-numbers/

struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut c1 = version1.split('.').collect::<Vec<&str>>();
        let mut c2 = version2.split('.').collect::<Vec<&str>>();
        while c1.len() < c2.len() {
            c1.push("0");
        }
        while c1.len() > c2.len() {
            c2.push("0");
        }
        for i in 0..c1.len() {
            let c1i = c1[i].parse::<u32>().unwrap();
            let c2i = c2[i].parse::<u32>().unwrap();
            if c1i != c2i {
                return if c1i < c2i { -1 } else { 1 };
            }
        }
        0
    }
}

fn main() {
    let result = Solution::compare_version("1.2".to_string(), "1.10".to_string());
    println!("{result}")
}
