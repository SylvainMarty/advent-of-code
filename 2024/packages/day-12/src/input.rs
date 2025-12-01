use utils::filesystem::read_lines;

pub fn get_input(filename: &str) -> Vec<Vec<char>> {
  let res = read_lines(format!("./packages/day-12/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.map_while(Result::ok);
      parse_input(lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input(lines: Vec<String>) -> Vec<Vec<char>> {
  let mut vec = Vec::new();
  for line in lines {
    vec.push(line.chars().collect::<Vec<char>>());
  }
  vec
}
