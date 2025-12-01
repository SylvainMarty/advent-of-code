use crate::functions::cound_valid_equations;

pub fn part_1(lines: &Vec<String>) -> i64 {
  const OPERATORS: [char; 2] = ['+','*'];
  cound_valid_equations(&lines, &OPERATORS.to_vec())
}

pub fn part_2(lines: &Vec<String>) -> i64 {
  const OPERATORS: [char; 3] = ['+','*','|'];
  cound_valid_equations(&lines, &OPERATORS.to_vec())
}

#[cfg(test)]
mod tests {
  use super::*;

  static TEST_LINES: [&str; 9] = [
    "190: 10 19",
    "3267: 81 40 27",
    "83: 17 5",
    "156: 15 6",
    "7290: 6 8 6 15",
    "161011: 16 10 13",
    "192: 17 8 14",
    "21037: 9 7 18 13",
    "292: 11 6 16 20",
  ];

  #[test]
  fn part_1_works() {
    let test_input = TEST_LINES.into_iter().map(|x| x.to_string()).collect();
    assert_eq!(part_1(&test_input), 3749);
  }

  #[test]
  fn part_2_works() {
    let test_input = TEST_LINES.into_iter().map(|x| x.to_string()).collect();
    assert_eq!(part_2(&test_input), 11387);
  }
}
