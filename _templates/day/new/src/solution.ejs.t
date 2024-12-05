---
to: packages/day-<%= dayNumber %>/src/solution.rs
---
// use crate::functions::my_function;

pub fn part_1(lines: Vec<String>) -> i32 {
  let mut result = 0;
  for line in lines {
    println!("{:?}", line);
  }
  result
}

pub fn part_2(lines: Vec<String>) -> i32 {
  let mut result = 0;
  for line in lines {
    println!("{:?}", line);
  }
  result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LINES: [&str; 2] = [
      "",
      "",
    ];

    #[test]
    fn part_1_works() {
      assert_eq!(part_1(TEST_LINES.into_iter().map(|x| x.to_string()).collect()), 143);
    }

    #[test]
    fn part_2_works() {
      assert_eq!(part_2(TEST_LINES.into_iter().map(|x| x.to_string()).collect()), 123);
    }
}
