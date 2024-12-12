use std::collections::HashSet;
use utils::vector::is_valid_coord;

pub fn compute_area_and_perimeter_for_part_1(
  dirs_x: &Vec<i32>,
  dirs_y: &Vec<i32>,
  grid: &Vec<Vec<char>>,
  accepted_positions: &mut HashSet<(usize, usize)>,
  area_and_perimeter: &mut (i64, i64),
  m: usize,
  n: usize,
  x: usize,
  y: usize,
  curr_label: char
) {
  if !is_valid_coord(x as i32, y as i32, m, n) || grid[x][y] != curr_label {
    area_and_perimeter.1 += 1;
    return;
  }
  if accepted_positions.contains(&(x, y)) {
    return;
  }
  accepted_positions.insert((x, y));
  area_and_perimeter.0 += 1;
  for k in 0..dirs_x.len() {
    let dir_x: usize = (x as i32 + dirs_x[k] as i32) as usize;
    let dir_y: usize = (y as i32 + dirs_y[k] as i32) as usize;
    
    compute_area_and_perimeter_for_part_1(
      dirs_x,
      dirs_y,
      grid,
      accepted_positions,
      area_and_perimeter,
      m,
      n,
      dir_x,
      dir_y,
      curr_label
    );
  }
  return;
}

pub struct Garden {
  pub grid: Vec<Vec<char>>,
  pub width: usize,
  pub height: usize,
}

struct Measures {
  area: usize,
  perimeter: usize,
}

impl Measures {
  fn new() -> Self {
    Measures {
      area: 0,
      perimeter: 0,
    }
  }
}

#[derive(PartialEq, Clone, Hash, Eq)]
enum Direction {
  North,
  South,
  East,
  West,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Fence {
  direction: Direction,
  row: usize,
  col: usize,
}

impl Fence {
  fn next(&mut self) {
    match self.direction {
      Direction::North | Direction::South => self.col += 1,
      Direction::East | Direction::West => self.row += 1,
    };
  }
  
  fn prev(&mut self) {
    match self.direction {
      Direction::North | Direction::South => self.col = self.col.wrapping_sub(1),
      Direction::East | Direction::West => self.row = self.row.wrapping_sub(1),
    };
  }
}

impl Garden {
  pub fn new(input: &str) -> Self {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();
    
    Self {
      grid,
      width,
      height,
    }
  }
  
  fn mesure_region(
    &mut self,
    row: usize,
    col: usize,
    measures: &mut Measures,
    visited: &mut Vec<Vec<bool>>,
  ) {
    if visited[row][col] {
      return;
    }
    // mark as visited
    visited[row][col] = true;
    
    let plant_type = self.grid[row][col];
    measures.area += 1;
    
    if row > 0 && self.grid[row - 1][col] == plant_type {
      self.mesure_region(row - 1, col, measures, visited);
    } else {
      measures.perimeter += 1;
    }
    
    if row + 1 < self.height && self.grid[row + 1][col] == plant_type {
      self.mesure_region(row + 1, col, measures, visited);
    } else {
      measures.perimeter += 1;
    }
    
    if col > 0 && self.grid[row][col - 1] == plant_type {
      self.mesure_region(row, col - 1, measures, visited);
    } else {
      measures.perimeter += 1;
    }
    
    if col + 1 < self.width && self.grid[row][col + 1] == plant_type {
      self.mesure_region(row, col + 1, measures, visited);
    } else {
      measures.perimeter += 1;
    }
  }
  
  pub fn mesure_area_and_fences(
    &mut self,
    row: usize,
    col: usize,
    area: &mut usize,
    visited: &mut Vec<Vec<bool>>,
    fences: &mut HashSet<Fence>,
  ) {
    if visited[row][col] {
      return;
    }
    
    // mark as visited
    visited[row][col] = true;
    
    let plant_type = self.grid[row][col];
    *area += 1;
    
    if row > 0 && self.grid[row - 1][col] == plant_type {
      self.mesure_area_and_fences(row - 1, col, area, visited, fences);
    } else {
      fences.insert(Fence {
        direction: Direction::North,
        row,
        col,
      });
    }
    
    if row + 1 < self.height && self.grid[row + 1][col] == plant_type {
      self.mesure_area_and_fences(row + 1, col, area, visited, fences);
    } else {
      fences.insert(Fence {
        direction: Direction::South,
        row,
        col,
      });
    }
    
    if col > 0 && self.grid[row][col - 1] == plant_type {
      self.mesure_area_and_fences(row, col - 1, area, visited, fences);
    } else {
      fences.insert(Fence {
        direction: Direction::West,
        row,
        col,
      });
    }
    
    if col + 1 < self.width && self.grid[row][col + 1] == plant_type {
      self.mesure_area_and_fences(row, col + 1, area, visited, fences);
    } else {
      fences.insert(Fence {
        direction: Direction::East,
        row,
        col,
      });
    }
  }
}

pub fn count_sides(fences: &mut HashSet<Fence>) -> usize {
  // counts fence sides and clears the hashset
  let mut sides = 0;
  
  while !fences.is_empty() {
    let fence_orig = fences.iter().next().unwrap().clone();
    
    let mut fence = fence_orig.clone();
    while fences.remove(&fence) {
      fence.next();
    }
    
    let mut fence = fence_orig;
    fence.prev();
    while fences.remove(&fence) {
      fence.prev();
    }
    
    sides += 1;
  }
  
  sides
}
