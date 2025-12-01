use utils::number::from_base10_to_base;

pub fn parse_equations(lines: &Vec<String>) -> Vec<(i64, Vec<i64>)> {
  let mut result = Vec::new();
  for line in lines {
    let mut numbers = line.split(':');
    let eq_result = match numbers.next().unwrap().parse::<i64>() {
      Ok(x) => x,
      Err(err) => {
        println!("errored_line={:#?}", line);
        println!("err={:#?}", err);
        panic!("Error parsing eq result");
      },
    };
    let operands = line.split_whitespace().filter_map(|x| x.parse::<i64>().ok()).collect::<Vec<i64>>();
    result.push((eq_result, operands));
  }
  result
}

pub fn cound_valid_equations(lines: &Vec<String>, operators: &Vec<char>) -> i64 {
  let op_len = operators.len();
  let equations = parse_equations(&lines);
  let mut result = 0;
  'root: for (eq, operands) in equations {
    let l = operands.len();
    let mut ops = vec![];
    for i in 0..op_len.pow((l - 1) as u32) {
      ops.push(
        format!("{:0>width$}", from_base10_to_base(i as i64, op_len as i64), width = l - 1)
          .chars()
          .map(|x| operators[x as usize - '0' as usize])
          .collect::<Vec<char>>()
      );
    }

    for op in ops {
      let mut curr_operands = operands.clone();
      let mut current = curr_operands.remove(0);
      for i in 1..l {
        let operand = curr_operands.remove(0);
        match op[i-1] {
          '+' => current += operand,
          '*' => current *= operand,
          '|' => current = format!("{}{}", current, operand).parse::<i64>().unwrap(),
          _ => panic!("Unknown operator"),
        }
      }
      if current == eq {
        result += eq;
        continue 'root;
      }
    }
  }
  result
}

