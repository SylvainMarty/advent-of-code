use std::collections::HashSet;
use utils::vector::is_valid_coord;

pub fn compute_area_and_perimeter_for_part_1(
  dirs_x: &Vec<i32>,
  dirs_y: &Vec<i32>,
  grid: &Vec<Vec<char>>,
  accepted_positions: &mut HashSet<(usize, usize)>,
  area_and_perimeter: &mut (i64, i64),
  m: usize,
  n: usize,
  x: usize,
  y: usize,
  curr_label: char
) {
  if !is_valid_coord(x as i32, y as i32, m, n) || grid[x][y] != curr_label {
    area_and_perimeter.1 += 1;
    return;
  }
  if accepted_positions.contains(&(x, y)) {
    return;
  }
  accepted_positions.insert((x, y));
  area_and_perimeter.0 += 1;
  for k in 0..dirs_x.len() {
    let dir_x: usize = (x as i32 + dirs_x[k] as i32) as usize;
    let dir_y: usize = (y as i32 + dirs_y[k] as i32) as usize;

    compute_area_and_perimeter_for_part_1(
      dirs_x,
      dirs_y,
      grid,
      accepted_positions,
      area_and_perimeter,
      m,
      n,
      dir_x,
      dir_y,
      curr_label
    );
  }
  return;
}
