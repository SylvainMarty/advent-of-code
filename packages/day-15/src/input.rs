use utils::filesystem::read_lines;

pub fn get_input(filename: &str) -> (Vec<Vec<char>>, Vec<(i32, i32)>) {
  let res = read_lines(format!("./packages/day-15/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.map_while(Result::ok);
      parse_input(lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input(lines: Vec<String>) -> (Vec<Vec<char>>, Vec<(i32, i32)>) {
  let mut grid = Vec::new();
  let mut actions = Vec::new();
  let mut add_actions = false;
  for line in lines {
    if line.is_empty() {
      add_actions = true;
      continue;
    }
    match add_actions {
      false => {
        grid.push(line.chars().collect());
      }
      true => {
        actions
          .append(&mut line.chars()
          .map(|c| match c {
            '^' => (-1, 0),
            'v' => (1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            _ => panic!("Unknown action: {}", c)
          })
          .collect());
      }
    }
  }
  (grid, actions)
}
