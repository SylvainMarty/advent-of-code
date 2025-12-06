// use crate::functions::my_function;

pub fn part_1(input: &(Vec<Vec<i64>>, Vec<char>)) -> i64 {
  let (grid, operators) = &input;
  let mut result = 0;
  for i in 0..grid.len() {
    result += grid[i]
      .iter()
      .skip(1)
      .fold(grid[i][0], |acc, &x| match operators[i] {
        '+' => acc + x,
        '*' => acc * x,
        _   => panic!("Unknown operator: {}", operators[i]),
      })
  }
  result
}

pub fn part_2(input: &(Vec<Vec<Option<i64>>>, Vec<char>)) -> i64 {
  let (grid, operators) = &input;
  let mut result = 0;
  for i in 0..grid.len() {
    result += grid[i]
      .iter()
      .skip(1)
      .fold(
        grid[i][0].unwrap_or(0),
        |acc, &x| {
          match operators[i] {
            '+' => acc + x.unwrap_or(0),
            '*' => acc * x.unwrap_or(1),
            _   => panic!("Unknown operator: {}", operators[i]),
          }
        }
      );
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::{parse_input_1, parse_input_2};

  static TEST_LINES: [&str; 4] = [
    "123 328  51 64 ",
    " 45 64  387 23 ",
    "  6 98  215 314",
    "*   +   *   +  ",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input_1(lines);
    assert_eq!(part_1(&test_input), 4277556);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input_2(lines);
    assert_eq!(part_2(&test_input), 3263827);
  }
}
