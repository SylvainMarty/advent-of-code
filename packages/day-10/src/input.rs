use utils::filesystem::read_lines;

pub fn get_input(filename: &str) -> Vec<Vec<i64>> {
  let res = read_lines(format!("./packages/day-10/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.flatten();
      parse_input(&lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input(lines: &Vec<String>) -> Vec<Vec<i64>> {
  let mut vec = Vec::new();
  for line in lines {
    vec.push(line.chars().map(|x| x.to_digit(10).unwrap() as i64).collect());
  }
  vec
}
