use utils::filesystem::read_lines;
use utils::solutions::execute_all;

fn main() {
  execute_all(vec![
    ("Part 1", part_1),
    // ("Part 2", part_2),
  ]);
}

fn part_1() -> i32 {
  #[derive(Clone, Debug)]
  struct Mul {
    a: Option<i32>,
    b: Option<i32>,
    is_started: bool,
    can_save_b: bool,
  }
  impl Mul {
      pub fn new() -> Self {
          Self {
              a: None,
              b: None,
              is_started: false,
              can_save_b: false,
          }
      }
      pub fn reset(&mut self) {
          self.a = None;
          self.b = None;
          self.is_started = false;
          self.can_save_b = false;
      }
      pub fn set_a(&mut self, val: i32) {
          self.a = Some(val);
      }
      pub fn set_b(&mut self, val: i32) {
          self.b = Some(val);
      }
  }

  // 1. mul(
  // 2. any number
  // 3. ,
  // 4. any number
  // 5. )
  let mut result = 0;
  for line in get_input() {
    // println!("{:?}", line);
    let mut mul = Mul::new();
    let mut subline = String::new();
    // TODO: use a sliding window
    for c in line.chars() {
      // println!("subline: {}", subline);
      subline.push(c);
      println!("subline: {}", subline);
      if subline == "mul(" && mul.is_started {
        println!("found mul(");
        // 1.
        mul.is_started = true;
      }
      if !mul.is_started {
        continue;
      }
      let try_int = subline.parse::<i32>();
      if try_int.is_ok() {
        // 2. & 4.
        if !mul.can_save_b {
          mul.set_a(try_int.unwrap());
          println!(" > found a: {}", mul.a.unwrap());
        } else {
          mul.set_b(try_int.unwrap());
          println!(" > found b: {}", mul.b.unwrap());
        }
      }
      if subline == "," && mul.a.is_some() {
        println!("  > found comma");
        // 3.
        mul.can_save_b = true;
      }
      if subline == ")" && mul.a.is_some() && mul.b.is_some() {
        println!("   > found )");
        println!("    > saving: {} = {}", subline, mul.a.unwrap() * mul.b.unwrap());
        // 5.
        result += mul.a.unwrap() * mul.b.unwrap();
        mul.reset();
        subline = String::new();
      }
    }
  }
  result
}

// fn part_2() -> i32 {
//   let vec = get_input();
//   let mut result = 0;
//   for line in vec {
//     println!("{:?}", line);
//   }
//   result
// }

fn get_input() -> Vec<String> {
  let mut vec = Vec::new();
  let res = read_lines("./packages/day-3/src/input-sample.txt");
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
