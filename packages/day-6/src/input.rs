use utils::filesystem::read_lines;

pub fn get_input(filename: &str) -> Vec<Vec<char>> {
  let mut grid = Vec::new();
  let res = read_lines(format!("./packages/day-6/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      for line in lines.map_while(Result::ok) {
        grid.push(line.chars().collect());
      }
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
  grid
}
