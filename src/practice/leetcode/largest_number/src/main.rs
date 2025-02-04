// https://leetcode.com/problems/largest-number/

struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        if nums.len() == 0 {
            return "0".to_string();
        }
        let mut nums_str: Vec<String> = Vec::with_capacity(nums.len());
        for &i in &nums {
            nums_str.push(i.to_string());
        }
        nums_str.sort_by(|a, b| {
            let ab = a.to_owned() + b;
            let ba = b.to_owned() + a;
            return ba.cmp(&ab);
        });
        if nums_str[0] == "0" {
            return "0".to_string();
        }
        nums_str.join("").to_string()
    }
}

fn main() {
    let result = Solution::largest_number(vec![10, 2]);
    println!("{result}")
}
