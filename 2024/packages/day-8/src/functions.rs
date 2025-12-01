use std::collections::{HashSet, HashMap};

use utils::vector::is_valid_coord;

fn get_all_letter_positions(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
  let letters = HashSet::from([
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
  ]);
  let m = grid.len();
  let n = grid[0].len();

  let mut letter_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
  for i in 0..m {
    for j in 0..n {
      let letter = grid[i][j];
      if !letters.contains(&letter) {
        continue;
      }
      if !letter_positions.contains_key(&letter) {
        letter_positions.insert(letter, vec![]);
      }
      letter_positions.get_mut(&letter).unwrap().push((i, j));
    }
  }
  letter_positions
}

pub fn get_unique_antinode_locations_count(grid: &Vec<Vec<char>>) -> i64 {
  let m = grid.len();
  let n = grid[0].len();

  let letter_positions = get_all_letter_positions(&grid);
  let mut antinode_positions = HashSet::new();
  for letter_pos in letter_positions.values() {
    let l = letter_pos.len();
    for a in 0..l {
      let (i, j) = letter_pos[a];
      for b in a+1..l {
        let (k, l) = letter_pos[b];
        let (x1, y1, x2, y2) = (i as i32, j as i32, k as i32, l as i32);
        let (v_x, v_y) = (x1 - x2, y1 - y2);
        let (x3, y3) = (x1 + v_x, y1 + v_y);
        if is_valid_coord(x3, y3, m, n) {
          antinode_positions.insert((x3, y3));
        }
        let (x4, y4) = (x2 - v_x, y2 - v_y);
        if is_valid_coord(x4, y4, m, n) {
          antinode_positions.insert((x4, y4));
        }
      }
    }
  }
  antinode_positions.len() as i64
}

pub fn get_unique_resonant_antinode_locations_count(grid: &Vec<Vec<char>>) -> i64 {
  let m = grid.len();
  let n = grid[0].len();

  let letter_positions = get_all_letter_positions(&grid);
  // let mut grad = grid.clone();
  let mut antinode_positions = HashSet::new();
  for letter_positions in letter_positions.values() {
    let l = letter_positions.len();
    for a in 0..l {
      let (i, j) = letter_positions[a];
      for b in 0..l {
        if a == b {
          continue;
        }
        let (k, l) = letter_positions[b];
        let (x1, y1, x2, y2) = (i as i32, j as i32, k as i32, l as i32);
        antinode_positions.insert((x1, y1));
        antinode_positions.insert((x2, y2));
        let (v_x, v_y) = (x1 - x2, y1 - y2);
        let mut x3 = x1 + v_x;
        let mut y3 = y1 + v_y;
        while is_valid_coord(x3, y3, m, n) {
          antinode_positions.insert((x3, y3));
          x3 = x3 + v_x;
          y3 = y3 + v_y;
        }
        let mut x4 = x2 - v_x;
        let mut y4 = y2 - v_y;
        while is_valid_coord(x4, y4, m, n) {
          antinode_positions.insert((x4, y4));
          x4 = x4 + v_x;
          y4 = y4 + v_y;
        }
      }
    }
  }
  antinode_positions.len() as i64
}
