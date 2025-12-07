use crate::functions::count_leaves_dfs;

pub fn part_1(grid: &Vec<Vec<char>>) -> i64 {
  let m = grid.len();
  let n = grid[0].len();
  let mut result = 0;
  for i in (2..m).step_by(2) {
    for j in 0..n {
      if grid[i][j] != '^' {
        continue;
      }
      let mut head: i32 = (i-2) as i32;
      while head >= 0 {
        if grid[head as usize][j] == '^' {
          break; // Splitter directly found above
        }
        // Detect if a direct beam is hitting the splitter at pos (i, j)
        if grid[head as usize][j] == 'S'
          || grid[head as usize][j-1] == '^'
          || grid[head as usize][j+1] == '^' {
          result += 1;
          break;
        }
        head -= 2;
      }
    }
  }
  result
}

pub fn part_2(grid: &Vec<Vec<char>>) -> i64 {
  use std::collections::HashMap;
  
  let m = grid.len();
  let n = grid[0].len();
  let x = 0;
  let mut y = 0;
  // Find the first splitter
  for j in 0..n {
    if grid[x][j] == 'S' {
      y = j;
      break;
    }
  }
  let mut memo: HashMap<(usize, usize), i64> = HashMap::new();
  1 + count_leaves_dfs(&grid, m, n, x, y, &mut memo)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 16] = [
    ".......S.......",
    "...............",
    ".......^.......",
    "...............",
    "......^.^......",
    "...............",
    ".....^.^.^.....",
    "...............",
    "....^.^...^....",
    "...............",
    "...^.^...^.^...",
    "...............",
    "..^...^.....^..",
    "...............",
    ".^.^.^.^.^...^.",
    "...............",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 21);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_2(&test_input), 40);
  }
}
