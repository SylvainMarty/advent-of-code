use crate::functions::{find_guard_position, get_guard_path, is_guard_on_loop};

pub fn part_1(grid: Vec<Vec<char>>) -> i32 {
  let m = grid.len();
  let n = grid[0].len();
  let (guard_x, guard_y) = find_guard_position(&grid);
  let (number_of_positions, _) = get_guard_path(&grid, m, n, guard_x, guard_y);
  number_of_positions
}

pub fn part_2(grid: Vec<Vec<char>>) -> i32 {
  let m = grid.len();
  let n = grid[0].len();
  let (guard_x, guard_y) = find_guard_position(&grid);
  let (_, guard_grid) = get_guard_path(&grid, m, n, guard_x, guard_y);
  let mut crate_on_path = guard_grid.clone().iter().map(|x| x.clone()).collect::<Vec<Vec<char>>>();
  let mut result = 0;
  for i in 0..m {
    for j in 0..n {
      if guard_grid[i][j] != 'X' {
        continue;
      }
      // let start = std::time::Instant::now();
      crate_on_path[i][j] = '#';
      if is_guard_on_loop(&crate_on_path, m, n, guard_x, guard_y) {
        result += 1;
      }
      crate_on_path[i][j] = 'X';
      // println!("i: {}, j: {}, elapsed: {:?}", i, j, start.elapsed());
    }
  }
  result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LINES: [&str; 10] = [
      "....#.....",
      ".........#",
      "..........",
      "..#.......",
      ".......#..",
      "..........",
      ".#..^.....",
      "........#.",
      "#.........",
      "......#...",
    ];

    #[test]
    fn part_1_works() {
      assert_eq!(part_1(TEST_LINES.into_iter().map(|x| x.chars().collect()).collect()), 41);
    }

    #[test]
    fn part_2_works() {
      assert_eq!(part_2(TEST_LINES.into_iter().map(|x| x.chars().collect()).collect()), 6);
    }
}
