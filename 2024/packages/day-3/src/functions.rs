#[derive(Clone, Debug)]
struct Mul {
  a: Option<i32>,
  b: Option<i32>,
  is_started: bool,
  can_save_b: bool,
}
impl Mul {
    pub fn new() -> Self {
        Self {
            a: None,
            b: None,
            is_started: false,
            can_save_b: false,
        }
    }
    pub fn reset(&mut self) {
        self.a = None;
        self.b = None;
        self.is_started = false;
        self.can_save_b = false;
    }
    pub fn set_a(&mut self, val: i32) {
        self.a = Some(val);
    }
    pub fn set_b(&mut self, val: i32) {
        self.b = Some(val);
    }
}

pub fn compute_mul_from_lines(lines: Vec<String>) -> i32 {
  const MUL_INSTR: [char; 4] = ['m', 'u', 'l', '('];
  let mut result = 0;
  for line in lines {
    let mut mul = Mul::new();
    let mut subline = String::new();
    let mut to_start = MUL_INSTR.to_vec();
    for c in line.chars() {
      if to_start.is_empty() {
        mul.is_started = true;
      }
      if !mul.is_started {
        if to_start[0] != c {
          // Resetting
          to_start = MUL_INSTR.to_vec();
          mul.reset();
          continue;
        }
        to_start.remove(0);
        continue;
      }
      if c == ',' && mul.a.is_some() {
        mul.can_save_b = true;
        subline = String::new();
        continue;
      }
      if c == ')' && mul.a.is_some() && mul.b.is_some() {
        result += mul.a.unwrap() * mul.b.unwrap();
        // Resetting
        mul.reset();
        subline = String::new();
        to_start = MUL_INSTR.to_vec();
        continue;
      }
      subline.push(c);
      let try_int = subline.parse::<i32>();
      if try_int.is_ok() {
        if !mul.can_save_b {
          mul.set_a(try_int.unwrap());
        } else {
          mul.set_b(try_int.unwrap());
        }
        continue;
      }
      // If we arrive here, something is wrong and we reset
      to_start = MUL_INSTR.to_vec();
      mul.reset();
      subline = String::new();
    }
  }
  result
}