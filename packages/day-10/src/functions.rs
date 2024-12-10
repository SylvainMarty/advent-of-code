use utils::vector::is_valid_coord;

pub fn count_trailhead_recursive(dirs_x: &Vec<i32>, dirs_y: &Vec<i32>, grid: &Vec<Vec<i64>>, m: usize, n: usize, x: usize, y: usize, curr_count: i64, curr_height: i64) -> i64 {
  println!(" > x: {}, y: {}, grid[x][y]: {}, curr_count: {}, curr_height: {}", x, y, grid[x][y], curr_count, curr_height);
  if grid[x][y] == 9 {
    return curr_count + 1;
  }
  let mut curr_count = curr_count;
  for k in 0..dirs_x.len() {
    let dir_x = x as i32 + dirs_x[k];
    let dir_y = y as i32 + dirs_y[k];

    if is_valid_coord(dir_x as i32, dir_y as i32, m, n) && grid[x][y] == curr_height+1 {
      curr_count = count_trailhead_recursive(
        dirs_x,
        dirs_y,
        grid,
        m,
        n,
        dir_x as usize,
        dir_y as usize,
        curr_count,
        curr_height+1
      );
    }
  }
  return curr_count;
}
