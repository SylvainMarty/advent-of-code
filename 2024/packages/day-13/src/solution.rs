use good_lp::{constraint, lp_solve, Solution, SolverModel, variables};

pub fn part_1(lines: &Vec<((f64, f64), (f64, f64), (f64, f64))>) -> i64 {
  let mut result: i64 = 0;
  for (button_a, button_b, prize) in lines {
    variables! {
        vars:
          0 <= a <= 100;
          0 <= b <= 100;
    }
    let solution = match vars
      .maximise(-3 * a - b)
      .using(lp_solve) 
      .with(constraint!(a * button_a.0 + b * button_b.0 == prize.0))
      .with(constraint!(a * button_a.1 + b * button_b.1 == prize.1))
      .solve() {
        Ok(solution) => solution,
        Err(_) => continue,
      };
    let x = solution.value(a).round() as i64;
    let y = solution.value(b).round() as i64;
    if x * button_a.0 as i64 + y * button_b.0 as i64 != prize.0 as i64
      || x * button_a.1 as i64 + y * button_b.1 as i64 != prize.1 as i64 {
      continue;
    }
    let nb_tokens = x * 3 + y;
    result += nb_tokens;
  }
  result
}

pub fn part_2(lines: &Vec<((f64, f64), (f64, f64), (f64, f64))>) -> i64 {
  let mut result: i64 = 0;
  for (button_a, button_b, prize) in lines {
    variables! {
        vars:
          0 <= a;
          0 <= b;
    }
    let new_prize = (prize.0 + 10000000000000_f64, prize.1 + 10000000000000_f64);
    let solution = match vars
      .maximise(-3 * a - b)
      .using(lp_solve) 
      .with(constraint!(a * button_a.0 + b * button_b.0 == new_prize.0))
      .with(constraint!(a * button_a.1 + b * button_b.1 == new_prize.1))
      .solve() {
        Ok(solution) => solution,
        Err(_) => continue,
      };
    let x = solution.value(a).round() as i64;
    let y = solution.value(b).round() as i64;
    if x * button_a.0 as i64 + y * button_b.0 as i64 != new_prize.0 as i64
      || x * button_a.1 as i64 + y * button_b.1 as i64 != new_prize.1 as i64 {
      continue;
    }
    let nb_tokens = x * 3 + y;
    result += nb_tokens;
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 15] = [
    "Button A: X+94, Y+34",
    "Button B: X+22, Y+67",
    "Prize: X=8400, Y=5400",
    "",
    "Button A: X+26, Y+66",
    "Button B: X+67, Y+21",
    "Prize: X=12748, Y=12176",
    "",
    "Button A: X+17, Y+86",
    "Button B: X+84, Y+37",
    "Prize: X=7870, Y=6450",
    "",
    "Button A: X+69, Y+23",
    "Button B: X+27, Y+71",
    "Prize: X=18641, Y=10279",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 480);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_2(&test_input), 875318608908);
  }
}
