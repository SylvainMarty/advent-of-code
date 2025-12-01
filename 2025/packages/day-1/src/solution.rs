use crate::functions::turn_dir;

pub fn part_1(lines: &Vec<String>) -> i64 {
  let mut head: i64 = 50;
  let mut answer: i64 = 0;
  for line in lines {
    let mut chars = line.chars();
    let turn = chars.next().unwrap().to_string();
    let turn_dir = turn_dir(&turn);
    let val: i64 = chars.collect::<String>().parse().unwrap();
    head = (head + (turn_dir * val)) % 100;
    if head == 0 {
      answer += 1;
    }
  }
  answer
}

pub fn part_2(lines: &Vec<String>) -> i64 {
  let mut head: i64 = 50;
  let mut answer: i64 = 0;
  for line in lines {
    let mut chars = line.chars();
    let turn = chars.next().unwrap().to_string();
    let turn_dir = turn_dir(&turn);
    let val: i64 = chars.collect::<String>().parse().unwrap();

    for _ in 0..val {
      head = (head + (1 * turn_dir)) % 100;
      if head == 0 {
        answer += 1;
      }
    }
  }
  answer
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 10] = [
    "L68",
    "L30",
    "R48",
    "L5",
    "R60",
    "L55",
    "L1",
    "L99",
    "R14",
    "L82",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 3);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_2(&test_input), 6);
  }
}
