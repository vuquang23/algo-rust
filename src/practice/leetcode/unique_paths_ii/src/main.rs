struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.len() == 0 {
            return 0;
        }
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        if obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }

        let mut dp = vec![vec![0 as i32; n]; m];
        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    dp[i][j] = 1;
                    continue;
                }
                if obstacle_grid[i][j] == 1 {
                    continue;
                }

                for (u, v) in vec![(-1, 0), (0, -1)] {
                    let zi = i as i32 + u;
                    let zj = j as i32 + v;
                    if zi < 0 || zj < 0 {
                        continue;
                    }
                    dp[i][j] += dp[zi as usize][zj as usize];
                }
            }
        }
        dp[m - 1][n - 1]
    }
}

fn main() {
    let result =
        Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]);
    println!("{result}")
}
