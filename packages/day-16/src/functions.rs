use std::slice::Iter;

use utils::vector::is_valid_coord;

pub fn find_start_end_pos(grid: &Vec<Vec<char>>) -> (usize, usize) {
  let n = grid.len();
  let m = grid[0].len();
  for i in 0..n {
    for j in 0..m {
      if grid[i][j] == 'S' {
        return (i, j);
      }
    }
  }
  panic!("No start position found");
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
  North,
  South,
  East,
  West,
}
impl Direction {
  pub fn iter() -> Iter<'static, Direction> {
      static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
      DIRECTIONS.iter()
  }

  pub fn nexts(&self) -> Vec<Direction> {
    let mut dirs = vec![self.clone()];
    let opposed_dir = self.get_opposite();
    for dir in Direction::iter() {
      if dir == self {
        continue;
      }
      if *dir == opposed_dir {
        continue;
      }
      dirs.push(*dir);
    }
    dirs
  }

  pub fn get_opposite(&self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::South => Direction::North,
      Direction::East => Direction::West,
      Direction::West => Direction::East,
    }
  }

  pub fn to_vector(&self) -> (i32, i32) {
    match self {
      Direction::North => (-1, 0),
      Direction::South => (1, 0),
      Direction::East => (0, 1),
      Direction::West => (0, -1),
    }
  }

  pub fn get_next_pos(&self, pos: (usize, usize)) -> (usize, usize) {
    let (x, y) = pos;
    let (dx, dy) = self.to_vector();
    // println!("     | get_next_pos(): {:?}", ((*x as i32 + dx) as usize, (*y as i32 + dy) as usize));
    ((x as i32 + dx) as usize, (y as i32 + dy) as usize)
  }
}

// Facing east by default  (0, 1)
pub fn get_lowest_score_path(
  grid: &Vec<Vec<char>>,
  from_pos: &(usize, usize),
  direction: Direction,
  curr_score: i64
) -> i64 {
  println!("> from_pos: {from_pos:?}, direction: {direction:?}");
  const SCORE_STRAIGHT: i64 = 1;
  const SCORE_ROTATE: i64 = 1000;
  let mut score = curr_score;
  println!("  > {:?}", direction.nexts());
  for next_dir in direction.nexts() {
    print!("   > next_dir: {next_dir:?}, get_next_pos(): {:?}, ", next_dir.get_next_pos(*from_pos));
    let (next_pos_x, next_pos_y) = next_dir.get_next_pos(*from_pos);
    let item = grid[next_pos_x][next_pos_y];
    println!("item: {item}, next_pos: ({next_pos_x:?}, {next_pos_y:?})");
    if item == '#' {
      continue;
    }
    let new_score: i64;
    if item == 'E' {
      new_score = curr_score + SCORE_STRAIGHT;
    } else {
      new_score = get_lowest_score_path(
        grid,
        &(next_pos_x, next_pos_y),
        next_dir,
        curr_score + SCORE_ROTATE
      );
    }
    score = score.min(new_score);
  }
  score
}
