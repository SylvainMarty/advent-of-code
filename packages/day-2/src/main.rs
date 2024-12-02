use utils::filesystem::read_lines;
use utils::solutions::execute_all;

fn main() {
  execute_all(vec![
    ("Part 1", part_1),
    ("Part 2", part_2),
  ]);
}

fn part_1() -> i32 {
  let vec = get_input();
  let mut result = 0;
  for line in vec.clone() {
    let mut is_line_safe = true;
    let direction = line[0] < line[1];
    for i in 1..line.len() {
      if !is_safe(direction, line[i-1], line[i]) {
        is_line_safe = false;
        break;
      }
    }
    if is_line_safe {
      result += 1;
    }
  }
  result
}

fn part_2() -> i32 {
  let vec = get_input();
  // Super mega dumb quadratic solution but hec I'm tired
  let mut result = 0;
  for line in vec {
    let mut safe = false;
    for i in 0..line.len() {
      let mut cloned_line = line.clone();
      cloned_line.remove(i);
      if is_line_safe(cloned_line) {
        safe = true;
        break;
      }
    }
    if safe {
      result += 1;
    }
  }
  result
}

fn is_line_safe(line: Vec<i32>) -> bool {
  let mut is_line_safe = true;
  let direction = line[0] < line[1];
  for i in 1..line.len() {
    if !is_safe(direction, line[i-1], line[i]) {
      is_line_safe = false;
      break;
    }
  }
  is_line_safe
}

fn is_safe(direction: bool, prev: i32, curr: i32) -> bool {
  if direction != (prev < curr) {
    return false;
  }
  let diff = (curr - prev).abs();
  if diff < 1 || diff > 3 {
    return false;
  }
  true
}

fn get_input() -> Vec<Vec<i32>> {
  let mut vec = Vec::new();
  let res = read_lines("./packages/day-2/src/input.txt");
  match res {
    Ok(lines) => {
      for line in lines.flatten() {
        let mut nums = Vec::new();
        for num in line.split_whitespace() {
          nums.push(num.parse::<i32>().unwrap());
        }
        vec.push(nums);
      }
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
  vec
}
