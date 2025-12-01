use utils::vector::convert_lines_to_grid;

use crate::functions::{get_unique_antinode_locations_count, get_unique_resonant_antinode_locations_count};

pub fn part_1(lines: &Vec<String>) -> i64 {
  let grid = convert_lines_to_grid(&lines);
  get_unique_antinode_locations_count(&grid)
}

pub fn part_2(lines: &Vec<String>) -> i64 {
  let grid = convert_lines_to_grid(&lines);
  get_unique_resonant_antinode_locations_count(&grid)
}

#[cfg(test)]
mod tests {
  use super::*;

  static TEST_LINES: [&str; 12] = [
    "............",
    "........0...",
    ".....0......",
    ".......0....",
    "....0.......",
    "......A.....",
    "............",
    "............",
    "........A...",
    ".........A..",
    "............",
    "............",
  ];

  #[test]
  fn part_1_works() {
    let test_input = TEST_LINES.into_iter().map(|x| x.to_string()).collect();
    assert_eq!(part_1(&test_input), 14);
  }

  #[test]
  fn part_2_works() {
    let test_input = TEST_LINES.into_iter().map(|x| x.to_string()).collect();
    assert_eq!(part_2(&test_input), 34);
  }
}
