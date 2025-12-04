use utils::solutions::execute_all;

mod input;
mod functions;
mod solution;
use solution::*;

use crate::input::get_input;

fn main() {
  let vec = get_input("input");
  let mut vec2 = vec.clone();
  let pt1 = move || part_1(&vec);
  let pt2 = move || part_2(&mut vec2);
  execute_all(vec![
    ("Part 1", Box::new(pt1)),
    ("Part 2", Box::new(pt2)),
  ]);
}
