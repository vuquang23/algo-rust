struct Solution;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut neg = 0 as i32;
        let mut pos = 0 as i32;
        for &v in &nums {
            if v >= 0 {
                break;
            }
            neg += 1;
        }
        for i in 0..nums.len() {
            if nums[nums.len() - 1 - i] <= 0 {
                break;
            }
            pos += 1
        }
        if neg > pos {
            neg
        } else {
            pos
        }
    }
}

fn main() {
    println!("Hello, world!");
}
