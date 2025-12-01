use utils::filesystem::read_lines;

pub fn get_input(filename: &str) -> Vec<String> {
  let res = read_lines(format!("./packages/day-1/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.map_while(Result::ok);
      parse_input(lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input(lines: Vec<String>) -> Vec<String> {
  let mut vec = Vec::new();
  for line in lines {
    vec.push(line.to_string());
  }
  vec
}
