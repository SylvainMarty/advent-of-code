use utils::filesystem::read_lines;

pub fn get_input(filename: &str) -> Vec<(String, String)> {
  let res = read_lines(format!("./packages/day-2/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.map_while(Result::ok);
      parse_input(lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input(lines: Vec<String>) -> Vec<(String, String)> {
  let mut vec = Vec::new();
  for line in lines {
    for num in line.split(',') {
      let mut nums = num.split('-');
      let start = nums.next().unwrap().to_string();
      let end = nums.next().unwrap().to_string();
      vec.push((start, end));
    }
  }
  vec
}
