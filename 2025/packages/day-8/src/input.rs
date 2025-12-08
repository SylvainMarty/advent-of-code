use utils::filesystem::read_lines;

pub fn get_input(filename: &str) -> Vec<(i64, i64, i64)> {
  let res = read_lines(format!("./packages/day-8/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.map_while(Result::ok);
      parse_input(lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input(lines: Vec<String>) -> Vec<(i64, i64, i64)> {
  let mut vec = Vec::new();
  for line in lines {
    let mut split = line.split(',');
    vec.push((
      split.next().unwrap().parse::<i64>().unwrap(),
      split.next().unwrap().parse::<i64>().unwrap(),
      split.next().unwrap().parse::<i64>().unwrap(),
    ));
  }
  vec
}
