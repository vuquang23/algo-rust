// https://leetcode.com/problems/number-of-islands

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
        fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, u: (usize, usize)) {
            visited[u.0][u.1] = true;
            for pair in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let x = u.0 as i32 + pair.0;
                let y = u.1 as i32 + pair.1;
                if x < 0 || x >= visited.len() as i32 || y < 0 || y >= visited[0].len() as i32 {
                    continue;
                }
                if visited[x as usize][y as usize] || grid[x as usize][y as usize] != '1' {
                    continue;
                }
                dfs(grid, visited, (x as usize, y as usize));
            }
        }
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] != '1' || visited[i][j] {
                    continue;
                }
                dfs(&grid, &mut visited, (i, j));
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let result = Solution::num_islands(vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ]);
    println!("{result}")
}
