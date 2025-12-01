use crate::functions::{apply_action_on_grid_if_possible, apply_action_on_grid_if_possible_p2, get_robot_position};

pub fn part_1(grid_and_actions: &(Vec<Vec<char>>, Vec<(i32, i32)>)) -> i64 {
  let mut grid = grid_and_actions.0.clone();
  let mut robot_pos = get_robot_position(&grid)
    .expect("Robot should be found");
  let m = grid.len();
  let n = grid[0].len();
  for action in grid_and_actions.1.iter() {
    apply_action_on_grid_if_possible(&mut grid, &mut robot_pos, action.clone(), '@');
  }
  let mut result: i64 = 0;
  for i in 1..m-1 {
    for j in 1..n-1 {
      if grid[i][j] == 'O' {
        result += 100 * i as i64 + j as i64;
      }
    }
  }
  result
}

pub fn part_2(grid_and_actions: &(Vec<Vec<char>>, Vec<(i32, i32)>)) -> i64 {
  let m = grid_and_actions.0.len();
  let n = grid_and_actions.0[0].len() * 2;
  let mut grid: Vec<Vec<char>> = vec![vec!['.'; n]; m];
  for i in 0..m {
    for j in (0..n).step_by(2) {
      let mut first_char = grid_and_actions.0[i][j/2];
      let mut second_char = first_char.clone();
      if first_char == 'O' {
        first_char = '[';
        second_char = ']';
      } else if first_char == '@' {
        second_char = '.';
      }
      grid[i][j] = first_char;
      grid[i][j+1] = second_char;
    }
  }
  println!("Grid before applying actions:");
  for gr in grid.iter() {
    println!("{:?}", gr);
  }
  let mut robot_pos = get_robot_position(&grid)
    .expect("Robot should be found");
  for action in grid_and_actions.1.iter() {
    apply_action_on_grid_if_possible_p2(&mut grid, &mut robot_pos, action.clone(), '@', false);
  }
  println!("Grid after applying actions:");
  for gr in grid.iter() {
    println!("{:?}", gr);
  }
  let mut result: i64 = 0;
  for i in 2..m-2 {
    for j in 2..n-2 {
      if grid[i][j] == '[' {
        result += 100 * i as i64 + j as i64;
      }
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES1: [&str; 10] = [
    "########",
    "#..O.O.#",
    "##@.O..#",
    "#...O..#",
    "#.#.O..#",
    "#...O..#",
    "#......#",
    "########",
    "",
    "<^^>>>vv<v>>v<<",
  ];
  static TEST_LINES2: [&str; 21] = [
    "##########",
    "#..O..O.O#",
    "#......O.#",
    "#.OO..O.O#",
    "#..O@..O.#",
    "#O#..O...#",
    "#O..O..O.#",
    "#.OO.O.OO#",
    "#....O...#",
    "##########",
    "",
    "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
    "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
    "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
    "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
    "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
    "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
    ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
    "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
    "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
    "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
  ];

  #[test]
  fn part_1_works_with_smaller_input() {
    let lines = TEST_LINES1.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 2028);
  }

  #[test]
  fn part_1_works_with_larger_input() {
    let lines = TEST_LINES2.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 10092);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES2.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_2(&test_input), 9021);
  }
}
