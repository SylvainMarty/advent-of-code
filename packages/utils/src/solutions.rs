use std::time::Duration;

fn execute_solution(name: &str, closure: fn() -> i32) -> (&str, i32, Duration) {
  let start = std::time::Instant::now();
  let res = closure();
  (name, res, start.elapsed())
}

pub fn execute_all(closures: Vec<(&str, fn() -> i32)>) {
  let start = std::time::Instant::now();
  let mut results = Vec::new();
  for (name, closure) in closures {
    results.push(execute_solution(name, closure));
  }
  let total_duration = start.elapsed();
  for (name, result, duration) in results {
    println!("{} result: {:?}", name, result);
    println!("{} done in: {:?}", name, duration);
  }
  println!("Total done in: {:?}", total_duration);
}
