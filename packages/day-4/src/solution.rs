use crate::functions::{get_xmas_count_from_lines_all_axis, get_xmas_count_from_lines_in_diagonals};

pub fn part_1(lines: Vec<String>) -> i64 {
  get_xmas_count_from_lines_all_axis(lines) as i64
}

#[cfg(test)]
mod tests_part_1 {
    use super::*;

    #[test]
    fn it_works() {
      let lines = vec![
        "MMMSXXMASM".to_string(),
        "MSAMXMSMSA".to_string(),
        "AMXSXMAAMM".to_string(),
        "MSAMASMSMX".to_string(),
        "XMASAMXAMM".to_string(),
        "XXAMMXXAMA".to_string(),
        "SMSMSASXSS".to_string(),
        "SAXAMASAAA".to_string(),
        "MAMMMXMMMM".to_string(),
        "MXMXAXMASX".to_string(),
      ];
      assert_eq!(part_1(lines), 18);
    }
}

pub fn part_2(lines: Vec<String>) -> i64 {
  get_xmas_count_from_lines_in_diagonals(lines) as i64
}

#[cfg(test)]
mod tests_part_2 {
    use super::*;

    #[test]
    fn it_works() {
      let lines = vec![
        "MMMSXXMASM".to_string(),
        "MSAMXMSMSA".to_string(),
        "AMXSXMAAMM".to_string(),
        "MSAMASMSMX".to_string(),
        "XMASAMXAMM".to_string(),
        "XXAMMXXAMA".to_string(),
        "SMSMSASXSS".to_string(),
        "SAXAMASAAA".to_string(),
        "MAMMMXMMMM".to_string(),
        "MXMXAXMASX".to_string(),
      ];
      assert_eq!(part_2(lines), 9);
    }
}
