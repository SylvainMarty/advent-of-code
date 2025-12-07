use std::collections::HashMap;

pub fn count_leaves_dfs(
    grid: &Vec<Vec<char>>,
    m: usize,
    n: usize,
    x: usize,
    y: usize,
    memo: &mut HashMap<(usize, usize), i64>
) -> i64 {
    // Check memo first
    if let Some(&cached) = memo.get(&(x, y)) {
        return cached;
    }

    if x == m - 1 {
        return 0;
    }

    let result = match grid[x + 1][y] {
        '^' => {
            // This splitter adds 1, plus all splits from both branches
            1 + count_leaves_dfs(grid, m, n, x + 1, y + 1, memo)
              + count_leaves_dfs(grid, m, n, x + 1, y - 1, memo)
        }
        '.' => {
            count_leaves_dfs(grid, m, n, x + 1, y, memo)
        }
        _ => 0
    };

    memo.insert((x, y), result);
    result
}
