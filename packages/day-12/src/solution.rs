use std::collections::HashSet;
use crate::functions::compute_area_and_perimeter_for_part_1;

pub fn part_1(grid: &Vec<Vec<char>>) -> i64 {
  let m = grid.len();
  let n = grid[0].len();
  let dirs_x = vec![-1, 0, 1, 0];
  let dirs_y = vec![0, 1, 0, -1];
  let mut result: i64 = 0;
  let mut accepted_positions = HashSet::new();
  for i in 0..m {
    for j in 0..n {
      if accepted_positions.contains(&(i, j)) {
        continue;
      }
      let mut area_and_perimeter = (0, 0);
      compute_area_and_perimeter_for_part_1(
        &dirs_x,
        &dirs_y,
        grid,
        &mut accepted_positions,
        &mut area_and_perimeter,
        m,
        n,
        i,
        j,
        grid[i][j]
      );
      result += area_and_perimeter.0 * area_and_perimeter.1;
    }
  }
  result
}

pub fn part_2(grid: &Vec<Vec<char>>) -> i64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 10] = [
    "RRRRIICCFF",
    "RRRRIICCCF",
    "VVRRRCCFFF",
    "VVRCCCJFFF",
    "VVVVCJJCFE",
    "VVIVCCJJEE",
    "VVIIICJJEE",
    "MIIIIIJJEE",
    "MIIISIJEEE",
    "MMMISSJEEE",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 1930);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_2(&test_input), 1206);
  }
}
