use std::collections::HashSet;

use crate::functions::{find_guard_position,is_inside_coord};

pub fn part_1(grid: Vec<Vec<char>>) -> i32 {
  let (guard_x, guard_y) = find_guard_position(grid.clone());
  let m = grid.len();
  let n = grid[0].len();
  // Guard always starts by going up
  const DIRECTIONS_X: [i32; 4] = [-1, 0, 1, 0];
  const DIRECTIONS_Y: [i32; 4] = [0, 1, 0, -1];
  let mut position_seen = HashSet::new();
  let mut dir_idx = 0;
  let mut next_x = guard_x;
  let mut next_y = guard_y;
  let mut result = 0;
  let mut grad = grid.clone().iter().map(|x| x.clone()).collect::<Vec<Vec<char>>>();
  while is_inside_coord(next_x as i32, next_y as i32, m, n) {
    println!("dir_idx: {}, next_x: {}, next_y: {}, cell: {:?}, result: {}", dir_idx, next_x, next_y, grid[next_x][next_y], result);
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
        result += 1;
        grad[next_x][next_y] = result.clone().to_string().chars().last().unwrap();
        position_seen.insert((next_x, next_y));
      }
    }
    next_x = ((next_x as i32) + DIRECTIONS_X[dir_idx]) as usize;
    next_y = ((next_y as i32) + DIRECTIONS_Y[dir_idx]) as usize;
  }
  for graad in grad {
    println!("{:?}", graad.iter().collect::<String>());
  }
  result
}

pub fn part_2(grid: Vec<Vec<char>>) -> i32 {
  let mut result: i32 = 0;
  // for line in grid {
  //   println!("{:?}", line);
  // }
  result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LINES: [&str; 10] = [
      "....#.....",
      ".........#",
      "..........",
      "..#.......",
      ".......#..",
      "..........",
      ".#..^.....",
      "........#.",
      "#.........",
      "......#...",
    ];

    #[test]
    fn part_1_works() {
      assert_eq!(part_1(TEST_LINES.into_iter().map(|x| x.chars().collect()).collect()), 41);
    }

    #[test]
    fn part_2_works() {
      assert_eq!(part_2(TEST_LINES.into_iter().map(|x| x.chars().collect()).collect()), 6);
    }
}
