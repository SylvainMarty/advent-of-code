pub fn convert_lines_to_grid(lines: &Vec<String>) -> Vec<Vec<char>> {
  let mut grid = Vec::new();
  for line in lines {
    grid.push(line.chars().collect());
  }
  grid
}

pub fn is_valid_coord(x: i32, y: i32, m: usize, n: usize) -> bool {
  x >= 0 && x < (m as i32) && y >= 0 && y < (n as i32)
}
