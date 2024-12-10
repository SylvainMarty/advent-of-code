use crate::functions::count_trailhead_recursive;

pub fn part_1(lines: &Vec<Vec<i64>>) -> i64 {
  let m = lines.len();
  let n = lines[0].len();
  let dirs_x = vec![-1, -1, -1, 0, 0, 1, 1, 1];
  let dirs_y = vec![-1, 0, 1, -1, 1, -1, 0, 1];
  let mut result = 0;
  for i in 0..m {
    for j in 0..n {
      if lines[i][j] != 0 {
        continue;
      }
      println!("i: {}, j: {}, lines[i][j]: {}", i, j, lines[i][j]);
      result += count_trailhead_recursive(
        &dirs_x,
        &dirs_y,
        lines,
        m,
        n,
        i,
        j,
        0,
        0
      );
    }
  }
  result
}

pub fn part_2(lines: &Vec<Vec<i64>>) -> i64 {
  0
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
    assert_eq!(part_2(&test_input), 123);
  }
}
