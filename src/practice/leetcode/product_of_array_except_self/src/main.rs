// https://leetcode.com/problems/product-of-array-except-self

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let product = nums
            .iter()
            .fold(1, |acc, &x| if x != 0 { acc * x } else { acc });
        let number_of_zeros = nums.iter().filter(|&&x| x == 0).count();
        let mut result = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            if number_of_zeros > 1 {
                result.push(0);
                continue;
            }
            if number_of_zeros == 1 {
                if nums[i] == 0 {
                    result.push(product);
                    continue;
                }
                result.push(0);
                continue;
            }
            result.push(product / nums[i]);
        }
        result
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let result = Solution::product_except_self(nums);
    println!("{:?}", result);
}
