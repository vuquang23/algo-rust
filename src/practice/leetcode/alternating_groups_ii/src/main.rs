struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let mut i = 0;
        let mut j = 0;
        let mut ans = 0;
        while i < colors.len() {
            let mut ok = true;
            while j - i + 1 + 1 <= k as usize {
                if colors[j % n] != colors[(j + 1) % n] {
                    j += 1;
                } else {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans += 1;
                i += 1;
            } else {
                j += 1;
                i = j;
            }
        }
        ans
    }
}

fn main() {
    let ans = Solution::number_of_alternating_groups(vec![0, 0, 1], 3);
    println!("{ans}")
}
