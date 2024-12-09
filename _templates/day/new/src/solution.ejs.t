---
to: packages/day-<%= dayNumber %>/src/solution.rs
---
// use crate::functions::my_function;

pub fn part_1(lines: &Vec<String>) -> i64 {
  0
}

pub fn part_2(lines: &Vec<String>) -> i64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 2] = [
    "",
    "",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(&lines);
    assert_eq!(part_1(&test_input), 143);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(&lines);
    assert_eq!(part_2(&test_input), 123);
  }
}
