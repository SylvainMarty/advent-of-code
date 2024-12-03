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

#[cfg(test)]
mod tests_part_1 {
    use super::*;

    #[test]
    fn it_works() {
      let lines = vec!["test1".to_string()];
      assert_eq!(part_1(lines), 0);
    }
}

pub fn part_2(lines: Vec<String>) -> i32 {
  let mut result = 0;
  for line in lines {
    println!("{:?}", line);
  }
  result
}

#[cfg(test)]
mod tests_part_2 {
    use super::*;

    #[test]
    fn it_works() {
      let lines = vec!["test2".to_string()];
      assert_eq!(part_2(lines), 0);
    }
}
