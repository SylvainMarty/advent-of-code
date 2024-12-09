// use crate::functions::my_function;

pub fn part_1(lines: &Vec<String>) -> i64 {
  let mut mem = Vec::new();
  let nums = lines.iter().next().unwrap().chars().map(|c| c.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();
  let mut free_slot_ids = Vec::new();
  println!("nums.len(): {:?}, nums: {:?}", nums.len(), nums);
  for id in 0..nums.len() {
    let num = nums[id as usize];
    let is_odd = (id % 2) != 0;
    let mut idx: i64 = (id as i64) / 2;
    if is_odd {
      idx = -1;
    }
    // println!("id: {}, idx: {}, num: {}, is_odd: {}", id, idx, num, is_odd);
    let l = mem.len();
    for i in 0..num as usize {
      // println!(" > i: {}", i);
      mem.push(idx);
      if is_odd {
        // save free space index
        free_slot_ids.push(i + l);
      }
    }
    // println!(" > mem: {:?}", mem);
  }
  println!("free_slot_ids.len(): {:?}, free_slot_ids: {:?}", free_slot_ids.len(), free_slot_ids);
  println!("mem: {mem:?}, diff: {}", mem.len() - free_slot_ids.len());
  let mut head = mem.len() - 1;
  while !free_slot_ids.is_empty() {
    let num = mem[head];
    if num == -1 {
      head -= 1;
      continue;
    }
    let id = free_slot_ids.remove(0);
    println!(" > id: {id}, num: {num}, head: {head}");
    mem[id] = num;
    mem[head] = -1;
    head -= 1;
  }
  println!("mem: {:?}", mem);
  mem.iter().enumerate().filter_map(|(idx, val)| {
    match val {
      -1 => None,
      _ => Some((idx as i64) * val)
    }
  }).sum()
}

pub fn part_2(lines: &Vec<String>) -> i64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  static TEST_LINES: [&str; 1] = [
    "2333133121414131402"
  ];

  #[test]
  fn part_1_works() {
    let test_input = TEST_LINES.into_iter().map(|x| x.to_string()).collect();
    assert_eq!(part_1(&test_input), 1928);
  }

  // #[test]
  // fn part_2_works() {
  //   let test_input = TEST_LINES.into_iter().map(|x| x.to_string()).collect();
  //   assert_eq!(part_2(&test_input), 123);
  // }
}
