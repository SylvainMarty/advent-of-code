// use crate::functions::my_function;

pub fn part_1(lines: &Vec<Vec<char>>) -> i64 {
  let m = lines.len();
  let n = lines[0].len();
  let mut result: i64 = 0;
  for i in 0..m {
    for j in 0..n {
      if lines[i][j] != '@' {
        continue;
      }
      let mut adjacent_roll_count = 0;
      // Check all 8 adjacent cells
      for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (1, 0), (1, -1), (0, -1), (0, 1), (1, 1)] {
        let x = i as i32 + dx;
        let y = j as i32 + dy;
        if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
          continue;
        }
        if lines[x as usize][y as usize] == '@' {
          adjacent_roll_count += 1;
        }
      }
      if adjacent_roll_count < 4 {
        result += 1;
      }
    }
  }
  result
}

pub fn part_2(lines: &mut Vec<Vec<char>>) -> i64 {
  let m = lines.len();
  let n = lines[0].len();
  let mut result: i64 = 0;
  let mut last_count: i64 = 1;
  // Keep iterating until there is no roll found
  while last_count > 0 {
    last_count = 0;
    for i in 0..m {
      for j in 0..n {
        if lines[i][j] != '@' {
          continue;
        }
        let mut adjacent_roll_count = 0;
        // Check all 8 adjacent cells
        for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (1, 0), (1, -1), (0, -1), (0, 1), (1, 1)] {
          let x = i as i32 + dx;
          let y = j as i32 + dy;
          if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
            continue;
          }
          if lines[x as usize][y as usize] == '@' {
            adjacent_roll_count += 1;
          }
        }
        if adjacent_roll_count < 4 {
          lines[i][j] = '.';
          last_count += 1;
        }
      }
    }
    result += last_count;
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 10] = [
    "..@@.@@@@.",
    "@@@.@.@.@@",
    "@@@@@.@.@@",
    "@.@@@@..@.",
    "@@.@@@@.@@",
    ".@@@@@@@.@",
    ".@.@.@.@@@",
    "@.@@@.@@@@",
    ".@@@@@@@@.",
    "@.@.@@@.@.",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 13);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let mut test_input = parse_input(lines);
    assert_eq!(part_2(&mut test_input), 43);
  }
}
