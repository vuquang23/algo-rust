struct Solution;

impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        let mut ret: i32 = 0;
        for n in nums.iter() {
            ret |= n;
        }
        ret
    }
}

fn main() {
    let result = Solution::maximum_xor(vec![3, 2, 4, 6]);
    print!("{result}")
}
