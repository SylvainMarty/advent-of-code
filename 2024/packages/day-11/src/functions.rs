use std::collections::HashMap;

pub fn get_stones_map(stones: &Vec<i64>) -> HashMap<i64, i64> {
  let mut stones_map: HashMap<i64, i64> = HashMap::new();
  for stone in stones {
    stones_map.insert(
      *stone,
      stones_map.get(&stone.clone()).unwrap_or(&0) + 1
    );
  }
  stones_map
}

pub fn blink_times(lines: &Vec<i64>, blinks: i64) -> i64 {
  let mut stones_map = get_stones_map(lines);
  for _ in 0..blinks {
    blink(&mut stones_map);
  }
  stones_map.values().sum()
}

fn blink(stones_map: &mut HashMap<i64, i64>) {
  for (stone, count) in stones_map.clone() {
    if count == 0 {
      continue;
    }
    if stone == 0 {
      stones_map.insert(1, stones_map.get(&1).unwrap_or(&0) + count);
      stones_map.insert(0, stones_map.get(&0).unwrap_or(&0) - count);
      continue;
    }
    if stone.to_string().len() % 2 == 0 {
      let stone_str = stone.to_string();
      let new_len = stone_str.len() / 2;
      let stone_1 = stone_str[0..new_len].parse::<i64>().unwrap();
      let stone_2 = stone_str[new_len..stone_str.len()].parse::<i64>().unwrap();
      stones_map.insert(stone_1, stones_map.get(&stone_1).unwrap_or(&0) + count);
      stones_map.insert(stone_2, stones_map.get(&stone_2).unwrap_or(&0) + count);
      stones_map.insert(stone, stones_map.get(&stone).unwrap_or(&0) - count);
      continue;
    }
    let new_stone = stone * 2024;
    stones_map.insert(new_stone, stones_map.get(&new_stone).unwrap_or(&0) + count);
    stones_map.insert(stone, stones_map.get(&stone).unwrap_or(&0) - count);
  }
}
