use utils::filesystem::read_lines;

pub fn get_input(filename: &str) -> Vec<((i64, i64), (i64, i64))> {
  let res = read_lines(format!("./packages/day-14/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.map_while(Result::ok);
      parse_input(lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input(lines: Vec<String>) -> Vec<((i64, i64), (i64, i64))> {
  let mut vec = Vec::new();
  for line in lines {
    let split = line.split_whitespace().collect::<Vec<&str>>();
    let ps = split.first().unwrap()[2..].split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let vs = split.last().unwrap()[2..].split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    vec.push((
      (ps[1], ps[0]),
      (vs[1], vs[0]),
    ));
  }
  vec
}
