use utils::filesystem::read_lines;

pub fn get_input_1(filename: &str) -> (Vec<Vec<i64>>, Vec<char>) {
  let res = read_lines(format!("./packages/day-6/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.map_while(Result::ok);
      parse_input_1(lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn get_input_2(filename: &str) -> (Vec<Vec<Option<i64>>>, Vec<char>) {
  let res = read_lines(format!("./packages/day-6/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.map_while(Result::ok);
      parse_input_2(lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input_1(lines: Vec<String>) -> (Vec<Vec<i64>>, Vec<char>) {
  let mut operators = Vec::new();
  let mut lines_iter = lines.iter();
  let mut col_head = 0;
  for c in lines_iter.next_back().unwrap().chars() {
    if c != ' ' {
      operators.push(c);
    }
  }

  let col_size = lines_iter.len();
  let row_size = operators.len();
  let mut grid = vec![vec![0i64; col_size]; row_size];

  for line in lines_iter {
    let mut row_head = 0;
    let mut str_num = String::new();
    for c in line.chars() {
      if c == ' ' {
        if str_num.len() > 0 {
          grid[row_head][col_head] = str_num.parse::<i64>().unwrap();
          row_head += 1;
          str_num = String::new();
        }
        continue;
      }
      str_num.push(c);
    }
    if str_num.len() > 0 {
      grid[row_head][col_head] = str_num.parse::<i64>().unwrap();
    }
    col_head += 1;
  }
  (grid, operators)
}

pub fn parse_input_2(lines: Vec<String>) -> (Vec<Vec<Option<i64>>>, Vec<char>) {
  let mut operators = Vec::new();
  let mut lines_iter = lines.iter();
  for c in lines_iter.next_back().unwrap().chars() {
    if c != ' ' {
      operators.push(c);
    }
  }
  operators.reverse();

  let line_chars = lines_iter.map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
  let col_size = line_chars.len();
  let row_size = operators.len();
  let mut grid = vec![vec![None; col_size]; row_size];
  let mut row_head = 0;
  let mut col_head = 0;

  let m = line_chars[0].len();
  let n = col_size;
  for y in (0..m).rev() {
    let mut str_num = String::new();
    for x in 0..n {
      let c = line_chars[x][y];
      if c != ' ' {
        str_num.push(c);
      }
    }
    if str_num.len() > 0 {
      grid[row_head][col_head] = Some(str_num.parse::<i64>().unwrap());
      col_head += 1;
    } else {
      row_head += 1;
      col_head = 0;
    }
  }
  (grid, operators)
}
