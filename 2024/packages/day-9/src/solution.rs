use crate::functions::{get_check_sum, get_mem_and_free_slots};

pub fn part_1(nums: &Vec<i64>) -> i64 {
  let (mem, free_slot_ids) = get_mem_and_free_slots(nums);
  let mut mem = mem;
  let mut free_slot_ids = free_slot_ids;
  let mut head = mem.len() - 1;
  while !free_slot_ids.is_empty() {
    let num = mem[head];
    if num == -1 {
      free_slot_ids.pop().unwrap();
      head -= 1;
      continue;
    }
    let id = free_slot_ids.remove(0);
    mem[id] = num;
    mem[head] = -1;
    head -= 1;
  }
  println!("{}", nums.len());
  get_check_sum(&mem)
}

pub fn part_2(nums: &Vec<i64>) -> i64 {
  #[derive(Debug)]
  struct File {
      id: usize,
      address: usize,
      length: usize,
      adjacent_free_space: usize,
  }

  let mut layout: Vec<File> = Vec::new();
  let mut block_count = 0;

  for (index, &length) in nums.iter().enumerate() {
    let length = length as usize;
    if index % 2 == 0 {
      layout.push(File {
        id: index / 2,
        address: block_count,
        length,
        adjacent_free_space: 0,
      });

      block_count += length;
    } else {
      let last_file = layout.last_mut().unwrap();
      last_file.adjacent_free_space = length;

      block_count += length;
    }
  }

  let mut moved_files: Vec<File> = Vec::with_capacity(layout.len());

  let mut max_length = None;
  let mut last_processed_id = layout.last().unwrap().id;

  'next_file: while let Some(mut file) = layout.pop() {
    if file.id > last_processed_id || max_length.is_some() && file.length >= max_length.unwrap() {
      moved_files.push(file);
      continue;
    }

    for i in 0..layout.len() {
      if layout[i].adjacent_free_space >= file.length {
        file.adjacent_free_space = layout[i].adjacent_free_space - file.length;
        file.address = layout[i].address + layout[i].length;
        layout[i].adjacent_free_space = 0;

        last_processed_id = file.id;

        layout.insert(i + 1, file);
        continue 'next_file;
      }
    }

    max_length = Some(file.length).into_iter().chain(max_length).min();
    moved_files.push(file);
  }

  moved_files
    .iter()
    .map(|file| file.id * (2 * file.address + file.length - 1) * file.length / 2)
    .sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 1] = [
    "2333133121414131402"
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(&lines);
    assert_eq!(part_1(&test_input), 1928);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(&lines);
    assert_eq!(part_2(&test_input), 2858);
  }
}
