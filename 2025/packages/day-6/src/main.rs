use utils::solutions::execute_all;

mod input;
mod functions;
mod solution;
use solution::*;

use crate::input::{get_input_1, get_input_2};

fn main() {
  let vec = get_input_1("input");
  let vec2 = get_input_2("input");
  let pt1 = move || part_1(&vec);
  let pt2 = move || part_2(&vec2);
  execute_all(vec![
    ("Part 1", Box::new(pt1)),
    ("Part 2", Box::new(pt2)),
  ]);
}
