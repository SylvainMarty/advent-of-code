use utils::filesystem::read_lines;
use crate::types::Point;

pub fn get_input(filename: &str) -> Vec<Point> {
  let res = read_lines(format!("./packages/day-9/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.map_while(Result::ok);
      parse_input(lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input(lines: Vec<String>) -> Vec<Point> {
  let mut vec = Vec::new();
  for line in lines {
    let mut split = line.split(',');
    vec.push((
      split.next().unwrap().parse::<i64>().unwrap(),
      split.next().unwrap().parse::<i64>().unwrap(),
    ));
  }
  vec
}
