use std::collections::{BTreeMap, HashSet};


pub fn part_1(input: &(BTreeMap<i64, i64>, Vec<i64>)) -> i64 {
  let mut result = 0;
  let (id_ranges, ids) = input;
  for id in ids {
    // Check ALL ranges that could contain this ID (start <= id)
    let is_valid = id_ranges
      .range(..=id) 
      .any(|(start, end)| start <= id && end >= id);
    if is_valid {
      result += 1;
    }
  }
  result
}

pub fn part_2(input: &(BTreeMap<i64, i64>, Vec<i64>)) -> i64 {
  let (id_ranges, _) = input;

  let mut result = 0;
  let mut last_end = 0;
  for (start, end) in id_ranges {
    if *start > last_end {
      result += end - start + 1;
      last_end = *end;
    } if *end > last_end {
      result += *end - last_end;
      last_end = *end;
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 11] = [
    "3-5",
    "10-14",
    "16-20",
    "12-18",
    "",
    "1",
    "5",
    "8",
    "11",
    "17",
    "32",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 3);
  }

  #[test]
  fn part_1_works_with_overlapping_ranges() {
    let lines = [
      "3-50",
      "10-15",
      "",
      "40",
    ].iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 1);
  }

  #[test]
  fn part_1_works_with_overlapping_range_with_same_start() {
    let lines = [
      "10-50",
      "10-15",
      "",
      "40",
    ].iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 1);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_2(&test_input), 14);
  }
}
