use std::time::Duration;

fn execute_solution(name: &str, closure: Box<dyn FnOnce() -> i64>) -> (&str, i64, Duration) {
  let start = std::time::Instant::now();
  let res = closure();
  (name, res, start.elapsed())
}

#[allow(clippy::println_empty_string, clippy::type_complexity)]
pub fn execute_all(closures: Vec<(&str, Box<dyn FnOnce() -> i64>)>) {
  let start = std::time::Instant::now();
  let mut results = Vec::new();
  for (name, closure) in closures {
    println!("{}: executing...", name);
    results.push(execute_solution(name, closure));
    println!("{}: done!", name);
  }
  let total_duration = start.elapsed();
  println!("");
  println!("=====================================");
  println!("============= Results ===============");
  println!("=====================================");
  println!("");
  for (name, result, duration) in results {
    println!("{} result: {:?}", name, result);
    println!("{} done in: {:?}", name, duration);
  }
  println!("Total done in: {:?}", total_duration);
}
