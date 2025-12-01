use utils::filesystem::read_lines;

pub fn get_input(filename: &str) -> Vec<String> {
  let mut vec = Vec::new();
  let res = read_lines(format!("./packages/day-8/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      for line in lines.map_while(Result::ok) {
        vec.push(line)
      }
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
  vec
}
