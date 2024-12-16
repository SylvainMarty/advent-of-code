pub fn get_robot_position(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
  let m = grid.len();
  let n = grid[0].len();
  for i in 1..m-1 {
    for j in 1..n-1 {
      if grid[i][j] == '@' {
        return Some((i, j));
      }
    }
  }
  None
}

pub fn apply_action_on_grid_if_possible(
  grid: &mut Vec<Vec<char>>,
  pos: &mut (usize, usize),
  action: (i32, i32),
  curr_item: char,
) {
  let next_pos_x = (pos.0 as i32 + action.0 as i32) as usize;
  let next_pos_y = (pos.1 as i32 + action.1 as i32) as usize;
  for item in ['#', 'O', '.'] {
    if item != grid[next_pos_x][next_pos_y] {
      continue;
    }
    match item {
      '#' => return,
      'O' => apply_action_on_grid_if_possible(grid, &mut (next_pos_x, next_pos_y), action, 'O'),
      '.' => {
        grid[pos.0][pos.1] = '.';
        grid[next_pos_x][next_pos_y] = curr_item;
        if curr_item == '@' {
          *pos = (next_pos_x, next_pos_y);
        }
      },
      _ => panic!("Unknown item: {}", grid[next_pos_x][next_pos_y])
    }
  }
}

pub fn apply_action_on_grid_if_possible_p2(
  grid: &mut Vec<Vec<char>>,
  pos: &mut (usize, usize),
  action: (i32, i32),
  curr_item: char,
  dry_run: bool,
) -> bool {
  if (action.0 == 1 || action.0 == -1) && (curr_item == '[' || curr_item == ']') && !dry_run {
    println!(" > Applying action {action:?} at pos {pos:?}, curr_item: {curr_item}, dry_run: {dry_run}");
  }
  let next_x = (pos.0 as i32 + action.0 as i32) as usize;
  let next_y = (pos.1 as i32 + action.1 as i32) as usize;
  for item in ['#', '[', ']', '.'] {
    if item != grid[next_x][next_y] {
      continue;
    }
    match item {
      '#' => return false,
      '[' => {
        if action.0 == 1 || action.0 == -1 {
          if apply_action_on_grid_if_possible_p2(grid, &mut (next_x, next_y), action, '[', true)
           && apply_action_on_grid_if_possible_p2(grid, &mut (next_x, next_y + 1), action, ']', true) {
            apply_action_on_grid_if_possible_p2(grid, &mut (next_x, next_y), action, '[', false);
            apply_action_on_grid_if_possible_p2(grid, &mut (next_x, next_y + 1), action, ']', false);
          }
        } else {
          apply_action_on_grid_if_possible_p2(grid, &mut (next_x, next_y), action, '[', false);
        }
      },
      ']' => {
        if action.0 == 1 || action.0 == -1 {
          if apply_action_on_grid_if_possible_p2(grid, &mut (next_x, next_y - 1), action, '[', true)
            && apply_action_on_grid_if_possible_p2(grid, &mut (next_x, next_y), action, ']', true) {
            apply_action_on_grid_if_possible_p2(grid, &mut (next_x, next_y - 1), action, '[', false);
            apply_action_on_grid_if_possible_p2(grid, &mut (next_x, next_y), action, ']', false);
          }
        } else {
          apply_action_on_grid_if_possible_p2(grid, &mut (next_x, next_y), action, ']', false);
        }
      },
      '.' => {
        if !dry_run {
          grid[pos.0][pos.1] = '.';
          grid[next_x][next_y] = curr_item;
          if curr_item == '@' {
            *pos = (next_x, next_y);
          }
        }
      },
      _ => panic!("Unknown item: {}", grid[next_x][next_y])
    }
  }
  true
}