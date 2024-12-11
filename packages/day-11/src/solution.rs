use crate::functions::blink_times;

pub fn part_1(lines: &Vec<i64>, blink_count: i64) -> i64 {
  blink_times(lines, blink_count)
}

pub fn part_2(lines: &Vec<i64>, blink_count: i64) -> i64 {
  blink_times(lines, blink_count)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 1] = [
    "125 17",
  ];

  #[test]
  fn part_1_works_blink_6() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(&lines);
    assert_eq!(part_1(&test_input, 6), 22);
  }

  #[test]
  fn part_1_works_blink_25() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(&lines);
    assert_eq!(part_1(&test_input, 25), 55312);
  }
}
