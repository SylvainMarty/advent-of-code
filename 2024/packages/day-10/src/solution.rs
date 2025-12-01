use std::collections::HashSet;

use crate::functions::{count_trailhead_recursive, count_trailhead_recursive_pt2};

pub fn part_1(grid: &Vec<Vec<i64>>) -> i64 {
  let m = grid.len();
  let n = grid[0].len();
  let dirs_x = vec![-1, 0, 1, 0];
  let dirs_y = vec![0, 1, 0, -1];
  let mut result: i64 = 0;
  for i in 0..m {
    for j in 0..n {
      if grid[i][j] != 0 {
        continue;
      }
      let mut nines = HashSet::new();
      count_trailhead_recursive(
        &dirs_x,
        &dirs_y,
        grid,
        &mut nines,
        m,
        n,
        i,
        j,
        0
      );
      result += nines.len() as i64;
    }
  }
  result
}

pub fn part_2(grid: &Vec<Vec<i64>>) -> i64 {
  let m = grid.len();
  let n = grid[0].len();
  let dirs_x = vec![-1, 0, 1, 0];
  let dirs_y = vec![0, 1, 0, -1];
  let mut result: i64 = 0;
  for i in 0..m {
    for j in 0..n {
      if grid[i][j] != 0 {
        continue;
      }
      let mut nines = Vec::new();
      count_trailhead_recursive_pt2(
        &dirs_x,
        &dirs_y,
        grid,
        &mut nines,
        m,
        n,
        i,
        j,
        0
      );
      result += nines.len() as i64;
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 8] = [
    "89010123",
    "78121874",
    "87430965",
    "96549874",
    "45678903",
    "32019012",
    "01329801",
    "10456732",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(&lines);
    assert_eq!(part_1(&test_input), 36);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(&lines);
    assert_eq!(part_2(&test_input), 81);
  }
}
