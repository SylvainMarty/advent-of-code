use crate::functions::has_repeated_digits;
use crate::functions::has_some_repeated_digits_slow;
use crate::functions::has_some_repeated_digits_fast;

pub fn part_1(lines: &Vec<(String, String)>) -> i64 {
  let mut result = 0;
  for line in lines {
    let start = line.0.parse::<i64>().unwrap();
    let end = line.1.parse::<i64>().unwrap();
    for num in start..=end {
      if has_repeated_digits(num.to_string().as_str()) {
        result += num;
      }
    }
  }
  result
}

pub fn part_2(lines: &Vec<(String, String)>) -> i64 {
  let mut result = 0;
  for line in lines {
    let start = line.0.parse::<i64>().unwrap();
    let end = line.1.parse::<i64>().unwrap();
    for num in start..=end {
      if has_some_repeated_digits_fast(num.to_string().as_str()) {
        result += num;
      }
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 1] = [
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 1227775554);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_2(&test_input), 4174379265);
  }
}
