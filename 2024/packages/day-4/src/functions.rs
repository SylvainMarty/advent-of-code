use utils::vector::{convert_lines_to_grid,is_valid_coord};

fn find_word(index: usize, word: &Vec<char>, grid: &Vec<Vec<char>>, l: usize, m: usize, n: usize, x: i32, y: i32, dir_x: i32, dir_y: i32) -> bool {
  // if word has been found
  if index == l {
    return true;
  }
  // if the current coordinate is valid and characters match, then check the next index
  if is_valid_coord(x, y, m, n) && word[index] == grid[x as usize][y as usize] {
    return find_word(index + 1, &word, grid, l, m, n, x + dir_x, y + dir_y, dir_x, dir_y);
  }

  return false;
}

pub fn get_xmas_count_from_lines_all_axis(lines: Vec<String>) -> i32 {
  let grid = convert_lines_to_grid(&lines);
  get_word_count_in_2d_grid_all_axis(&grid, "XMAS".to_string().chars().collect())
}

fn get_word_count_in_2d_grid_all_axis(grid: &Vec<Vec<char>>, word: Vec<char>) -> i32 {
  let mut result = 0;
  let l = word.len();
  let m = grid.len();
  let n = grid[0].len();

  // x and y are used to set the direction in which word needs to be searched.
  let x = [-1, -1, -1, 0, 0, 1, 1, 1];
  let y = [-1, 0, 1, -1, 1, -1, 0, 1];
  let directions = x.len();
  for i in 0..m {
    for j in 0..n {
      if word[0] != grid[i][j] {
        continue;
      }
      // Search in all directions
      for k in 0..directions {
        if find_word(0, &word, grid, l, m, n, i as i32, j as i32, x[k], y[k]) {
          result += 1;
        }
      }
    }
  }
  result
}

pub fn get_xmas_count_from_lines_in_diagonals(lines: Vec<String>) -> i32 {
  let grid = convert_lines_to_grid(&lines);
  get_word_count_in_2d_grid_in_diagonals(&grid, "MAS".to_string().chars().collect())
}

fn get_word_count_in_2d_grid_in_diagonals(grid: &Vec<Vec<char>>, word: Vec<char>) -> i32 {
  let mut result = 0;
  let l = word.len();
  if l % 2 == 0 {
    panic!("It only works with words with odd length");
  }
  let m = grid.len();
  let n = grid[0].len();
  let middle_char = word[(l/2) as usize];

  // x and y are used to set the direction in which word needs to be searched.
  let x = [1, 1, -1, -1];
  let y = [1, -1, 1, -1];
  let directions = x.len();
  for i in 0..m {
    for j in 0..n {
      // If not the middle char, we continue
      if middle_char != grid[i][j] {
        continue;
      }
      // Search in all directions
      let mut diag_found = 0;
      for k in 0..directions {
        if find_word(0, &word, grid, l, m, n, i as i32 + x[k], j as i32 + y[k], -x[k], -y[k]) {
          diag_found += 1;
        }
        if diag_found == 2 {
          result += 1;
          break;
        }
      }
    }
  }
  result
}
