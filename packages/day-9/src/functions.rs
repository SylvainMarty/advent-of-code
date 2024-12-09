pub fn get_check_sum(mem: &Vec<i64>) -> i64 {
  mem.iter().enumerate().filter_map(|(idx, val)| {
    match val {
      -1 => None,
      _ => Some((idx as i64) * val)
    }
  }).sum()
}

pub fn get_mem_and_free_slots(nums: &Vec<i64>) -> (Vec<i64>, Vec<usize>) {
  let mut mem = Vec::new();
  let mut free_slot_ids = Vec::new();
  for id in 0..nums.len() {
    let num = nums[id];
    let is_odd = (id % 2) != 0;
    let mut idx: i64 = (id as i64) / 2;
    if is_odd {
      idx = -1;
    }
    let l = mem.len();
    for i in 0..num as usize {
      mem.push(idx);
      if is_odd {
        // save free space index
        free_slot_ids.push(i + l);
      }
    }
  }
  (mem, free_slot_ids)
}
