pub fn from_base10_to_base(num: i64, base: i64) -> i64 {
  let mut ret = 0;
  let mut factor = 1;
  let mut num = num;
  while num > 0 {
      ret += num % base * factor;
      num /= base;
      factor *= 10;
  }
  ret
}
