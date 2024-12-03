---
to: packages/day-<%= dayNumber %>/src/main.rs
---
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
    println!("{:?}", line);
  }
  result
}

fn part_2() -> i32 {
  let vec = get_input();
  let mut result = 0;
  for line in vec {
    println!("{:?}", line);
  }
  result
}

fn get_input() -> Vec<String> {
  let mut vec = Vec::new();
  let res = read_lines("./packages/day-<%= dayNumber %>/src/input.txt");
  match res {
    Ok(lines) => {
      for line in lines.flatten() {
        vec.push(line)
      }
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
  vec
}
