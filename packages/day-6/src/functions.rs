pub fn is_inside_coord(x: i32, y: i32, m: usize, n: usize) -> bool {
  x >= 0 && x < (m as i32) && y >= 0 && y < (n as i32)
}

pub fn find_guard_position(grid: Vec<Vec<char>>) -> (usize, usize) {
  let m = grid.len();
  let n = grid[0].len();
  let mut x = 0;
  let mut y = 0;
  let mut found = false;
  'root: for i in 0..m {
    for j in 0..n {
      let cell = grid[i][j];
      if cell == '^' {
        found = true;
        x = i;
        y = j;
        break 'root;
      }
    }
  }
  if !found {
    panic!("No guard found in grid");
  }
  (x, y)
}
