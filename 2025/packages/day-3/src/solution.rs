pub fn part_1(lines: &Vec<String>) -> i64 {
  let mut result = 0;
  for line in lines {
    let mut left_max = 0;
    let mut right_max = 0;
    let line_len = line.len();
    let line_chars = line.chars().collect::<Vec<char>>();
    for i in 0..line_len {
      let num = line_chars[i].to_digit(10).unwrap();
      if i != line_len - 1 && num > left_max {
        left_max = num;
        right_max = line_chars[i+1].to_digit(10).unwrap();
      } else {
        right_max = right_max.max(num);
      }
    }
    result += format!("{}{}", left_max, right_max).parse::<i64>().unwrap();
  }
  result
}

pub fn part_2(lines: &Vec<String>) -> i64 {
  let mut result = 0;
  for line in lines {
    let line_len = line.len();
    let line_chars = line.chars().collect::<Vec<char>>();

    let mut jolt_nums: [i64; 12] = [0; 12];
    let mut jolt_nums_head = 0;
    let mut left = 0;

    while jolt_nums_head < 12 {
      let mut left_max = 0;
      for i in left ..= line_len - 12 + jolt_nums_head {
        let num: u32 = line_chars[i].to_digit(10).unwrap();
        if num > left_max {
          left_max = num;
          left = i + 1;
        }
      }
      jolt_nums[jolt_nums_head] = left_max as i64;
      jolt_nums_head += 1;
    }

    result += jolt_nums.iter().map(|x| x.to_string()).collect::<String>().parse::<i64>().unwrap();
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 4] = [
    "987654321111111",
    "811111111111119",
    "234234234234278",
    "818181911112111",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 357);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_2(&test_input), 3121910778619);
  }
}
