use utils::filesystem::read_lines;

pub fn get_input(filename: &str) -> Vec<i64> {
  let res = read_lines(format!("./packages/day-9/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.flatten();
      parse_input(&lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input(lines: &Vec<String>) -> Vec<i64> {
  lines[0].chars().map(|c| c.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>()
}
