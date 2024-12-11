pub fn count_stones_after_blinks(lines: &Vec<String>, blink_count: i64) -> i64 {
  let mut stones: Vec<String> = lines.clone();
  for _ in 0..blink_count {
    let mut new_stones: Vec<String> = Vec::new();
    for i in 0..stones.len() {
      let stone = stones[i].clone();
      if stone == "0" {
        new_stones.push("1".to_string());
        continue;
      }
      let stone_len = stone.len();
      if stone_len % 2 == 0 {
        new_stones.push(stone[0..stone_len/2].parse::<i64>().unwrap().to_string());
        new_stones.push(stone[stone_len/2..stone_len].parse::<i64>().unwrap().to_string());
        continue;
      }
      new_stones.push((stone.parse::<i64>().unwrap() * 2024).to_string());
    }
    stones = new_stones;
  }
  stones.len() as i64
}
