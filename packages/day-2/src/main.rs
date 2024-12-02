use utils::filesystem::read_lines;

fn main() {
  let start = std::time::Instant::now();
  let vec = get_input();
  // Part 1
  let mut result_part1 = 0;
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
      result_part1 += 1;
    }
  }
  let time_part_1 = start.elapsed();

  // Part 2
  let start_part2 = std::time::Instant::now();
  let mut result_part2 = 0;
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
      result_part2 += 1;
    }
  }
  let time_part_2 = start_part2.elapsed();

  // End
  println!("Result part 1: {}", result_part1);
  println!(" > Done part 1 in: {:?}", time_part_1);
  println!("Result part 2: {}", result_part2);
  println!(" > Done part 2 in: {:?}", time_part_2);
  println!("Total done in: {:?}", start.elapsed()); 
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
