use crate::functions::compute_mul_from_lines;

pub fn part_1(lines: Vec<String>) -> i32 {
  compute_mul_from_lines(lines)
}

#[cfg(test)]
mod tests_part_1 {
    use super::*;

    #[test]
    fn it_works() {
      let lines = vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()];
      assert_eq!(part_1(lines), 161);
    }
}

pub fn part_2(lines: Vec<String>) -> i32 {
  const CAN_DO_INSTR: [char; 4] = ['d', 'o', '(', ')'];
  const CANNOT_DO_INSTR: [char; 7] = ['d', 'o', 'n', '\'', 't', '(', ')'];
  let mut can_do_instr = CAN_DO_INSTR.to_vec();
  let mut cannot_do_instr = CANNOT_DO_INSTR.to_vec();
  let mut lines_to_process = Vec::new();
  let mut save_line = true;
  // First, we analyze the lines to extract the instructions between a do() and a don't()
  // for line in get_input("input") {
  for line in lines {
    let mut subline: String = String::new();
    for c in line.chars() {
      if !can_do_instr.is_empty() && can_do_instr[0] == c {
        can_do_instr.remove(0);
      } else if can_do_instr.len() != CAN_DO_INSTR.len() {
        can_do_instr = CAN_DO_INSTR.to_vec();
      }
      if !cannot_do_instr.is_empty() && cannot_do_instr[0] == c {
        cannot_do_instr.remove(0);
      } else if cannot_do_instr.len() != CANNOT_DO_INSTR.len() {
        cannot_do_instr = CANNOT_DO_INSTR.to_vec();
      }
      if can_do_instr.is_empty() {
        save_line = true;
      } else if cannot_do_instr.is_empty() {
        save_line = false;
      }
      if save_line {
        subline.push(c);
      } else {
        lines_to_process.push(subline);
        subline = String::new();
      }
    }
    if !subline.is_empty() {
      lines_to_process.push(subline);
    }
  }
  compute_mul_from_lines(lines_to_process)
}

#[cfg(test)]
mod tests_part_2 {
    use super::*;

    #[test]
    fn it_works() {
      let lines = vec!["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()];
      assert_eq!(part_2(lines), 48);
    }
}
