---
to: packages/day-<%= dayNumber %>/src/main.rs
---
use utils::solutions::execute_all;

mod input;
mod functions;
mod solution;
use solution::*;

use crate::input::get_input;

fn main() {
  let vec = get_input("input");
  let vec2 = vec.clone();
  let pt1 = move || part_1(&vec);
  let pt2 = move || part_2(&vec2);
  execute_all(vec![
    ("Part 1", Box::new(pt1)),
    ("Part 2", Box::new(pt2)),
  ]);
}
