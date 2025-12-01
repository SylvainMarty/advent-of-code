use std::collections::HashSet;

pub fn is_inside_coord(x: i32, y: i32, m: usize, n: usize) -> bool {
  x >= 0 && x < (m as i32) && y >= 0 && y < (n as i32)
}

pub fn find_guard_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
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

// pub fn get_guard_path(grid: &Vec<Vec<char>>, m: usize, n: usize, guard_x: usize, guard_y: usize) -> (HashSet<(usize, usize)>, Vec<Vec<char>>) {
pub fn get_guard_path(grid: &Vec<Vec<char>>, m: usize, n: usize, guard_x: usize, guard_y: usize) -> HashSet<(usize, usize)> {
  // Guard always starts by going up
  const DIRECTIONS_X: [i32; 4] = [-1, 0, 1, 0];
  const DIRECTIONS_Y: [i32; 4] = [0, 1, 0, -1];
  let mut position_seen = HashSet::new();
  let mut dir_idx = 0;
  let mut next_x = guard_x;
  let mut next_y = guard_y;
  while is_inside_coord(next_x as i32, next_y as i32, m, n) {
    let next_cell = grid[next_x][next_y];
    if next_cell == '#' {
      next_x = ((next_x as i32) - DIRECTIONS_X[dir_idx]) as usize;
      next_y = ((next_y as i32) - DIRECTIONS_Y[dir_idx]) as usize;
      dir_idx += 1;
      if dir_idx == 4 {
        dir_idx = 0;
      }
    } else {
      if !position_seen.contains(&(next_x, next_y)) {
        position_seen.insert((next_x, next_y));
      }
    }
    next_x = ((next_x as i32) + DIRECTIONS_X[dir_idx]) as usize;
    next_y = ((next_y as i32) + DIRECTIONS_Y[dir_idx]) as usize;
  }
  position_seen
}

pub fn is_guard_on_loop(grid: &Vec<Vec<char>>, m: usize, n: usize, guard_x: usize, guard_y: usize) -> bool {
  // Guard always starts by going up
  const DIRECTIONS_X: [i32; 4] = [-1, 0, 1, 0];
  const DIRECTIONS_Y: [i32; 4] = [0, 1, 0, -1];
  let mut position_seen = HashSet::new();
  let mut dir_idx = 0;
  let mut next_x = guard_x;
  let mut next_y = guard_y;
  while is_inside_coord(next_x as i32, next_y as i32, m, n) {
    let next_cell = grid[next_x][next_y];
    if next_cell == '#' {
      next_x = ((next_x as i32) - DIRECTIONS_X[dir_idx]) as usize;
      next_y = ((next_y as i32) - DIRECTIONS_Y[dir_idx]) as usize;
      dir_idx += 1;
      if dir_idx == 4 {
        dir_idx = 0;
      }
    } else if position_seen.contains(&(next_x, next_y, dir_idx)) {
      return true;
    }
    position_seen.insert((next_x, next_y, dir_idx));
    next_x = ((next_x as i32) + DIRECTIONS_X[dir_idx]) as usize;
    next_y = ((next_y as i32) + DIRECTIONS_Y[dir_idx]) as usize;
  }
  return false;
}
