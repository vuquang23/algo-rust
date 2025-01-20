// https://leetcode.com/problems/trapping-rain-water/

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        use std::cmp::min;
        if height.len() == 0 {
            return 0;
        }
        let mut lef: Vec<i32> = vec![];
        lef.resize(height.len(), -1);
        let mut rig: Vec<i32> = vec![];
        rig.resize(height.len(), -1);
        rig.resize(height.len(), -1);
        let mut stack: Vec<usize> = vec![];
        for i in 0..height.len() {
            while stack.len() > 0 {
                let top = stack[stack.len() - 1];
                if height[top] <= height[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.len() != 0 {
                lef[i] = stack[0] as i32;
            }
            stack.push(i);
        }
        while stack.len() > 0 {
            stack.pop();
        }
        for ii in 0..height.len() {
            let i = height.len() - 1 - ii;
            while stack.len() > 0 {
                let top = stack[stack.len() - 1];
                if height[top] <= height[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.len() != 0 {
                rig[i] = stack[0] as i32;
            }
            stack.push(i);
        }
        let mut result: i32 = 0;
        for i in 0..height.len() {
            if lef[i] < 0 || rig[i] < 0 {
                continue;
            }
            result += -height[i] + min(height[lef[i] as usize], height[rig[i] as usize]);
        }
        result
    }
}

fn main() {
    let result = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
    println!("{result}");
}
