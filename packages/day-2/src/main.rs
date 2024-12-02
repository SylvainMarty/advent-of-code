use utils::filesystem::read_lines;

fn main() {
  let start = std::time::Instant::now();
  let vec = get_input();
  // Part 1
  let mut result_part1 = 0;
  for line in vec.clone() {
    print!("{:?}", line);
    let mut is_safe = true;
    let direction = line[0] < line[1];
    for i in 1..line.len() {
      if direction != (line[i - 1] < line[i]) {
        is_safe = false;
        break;
      }
      let diff = (line[i] - line[i - 1]).abs();
      if diff < 1 || diff > 3 {
        is_safe = false;
        break;
      }
    }
    if is_safe {
      result_part1 += 1;
      println!(" > safe");
    } else {
      println!(" > unsafe");
    }
  }

  // Part 2
  let mut result_part2 = 0;
  for line in vec {
    print!("{:?}", line);
    let mut unsafe_tolerance = 1;
    let mut is_safe = true;
    let direction = line[0] < line[1];
    for i in 1..line.len() {
      if direction != (line[i - 1] < line[i]) {
        if unsafe_tolerance <= 0 {
          is_safe = false;
          break;
        }
        unsafe_tolerance -= 1;
      }
      let diff = (line[i] - line[i - 1]).abs();
      if diff < 1 || diff > 3 {
        if unsafe_tolerance <= 0 {
          is_safe = false;
          break;
        }
        unsafe_tolerance -= 1;
      }
    }
    if is_safe {
      result_part2 += 1;
      println!(" > safe");
    } else {
      println!(" > unsafe");
    }
  }

  // End
  println!("Result part 1: {}", result_part1);
  println!("Result part 2: {}", result_part2);
  eprintln!("Total done in: {:?}", start.elapsed()); 
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
