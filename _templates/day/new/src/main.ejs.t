---
to: packages/day-<%= dayNumber %>/src/main.rs
---
use utils::filesystem::read_lines;

fn main() {
  let start = std::time::Instant::now();
  // Part 1
  let vec = get_input();
  println!("{:?}", vec);

  // End
  eprintln!("Total done in: {:?}", start.elapsed()); 
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
