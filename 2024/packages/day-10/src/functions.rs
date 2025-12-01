use std::collections::HashSet;
use utils::vector::is_valid_coord;

pub fn count_trailhead_recursive(
  dirs_x: &Vec<i32>,
  dirs_y: &Vec<i32>,
  grid: &Vec<Vec<i64>>,
  curr_count: &mut HashSet<(usize, usize)>,
  m: usize,
  n: usize,
  x: usize,
  y: usize,
  curr_height: i64
) {
  if grid[x][y] == 9 {
    curr_count.insert((x, y));
    return;
  }
  for k in 0..dirs_x.len() {
    let dir_x: usize = (x as i32 + dirs_x[k] as i32) as usize;
    let dir_y: usize = (y as i32 + dirs_y[k] as i32) as usize;

    if is_valid_coord(dir_x as i32, dir_y as i32, m, n) && grid[dir_x][dir_y] == curr_height+1 {
      count_trailhead_recursive(
        dirs_x,
        dirs_y,
        grid,
        curr_count,
        m,
        n,
        dir_x,
        dir_y,
        curr_height+1
      );
    }
  }
  return;
}

pub fn count_trailhead_recursive_pt2(
  dirs_x: &Vec<i32>,
  dirs_y: &Vec<i32>,
  grid: &Vec<Vec<i64>>,
  curr_count: &mut Vec<(usize, usize)>,
  m: usize,
  n: usize,
  x: usize,
  y: usize,
  curr_height: i64
) {
  if grid[x][y] == 9 {
    curr_count.push((x, y));
    return;
  }
  for k in 0..dirs_x.len() {
    let dir_x: usize = (x as i32 + dirs_x[k] as i32) as usize;
    let dir_y: usize = (y as i32 + dirs_y[k] as i32) as usize;

    if is_valid_coord(dir_x as i32, dir_y as i32, m, n) && grid[dir_x][dir_y] == curr_height+1 {
      count_trailhead_recursive_pt2(
        dirs_x,
        dirs_y,
        grid,
        curr_count,
        m,
        n,
        dir_x,
        dir_y,
        curr_height+1
      );
    }
  }
  return;
}
