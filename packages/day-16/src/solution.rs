use crate::functions::{find_start_end_pos, get_lowest_score_path, Direction};

pub fn part_1(grid: &Vec<Vec<char>>) -> i64 {
  let start_pos = find_start_end_pos(&grid);
  println!("{:?}", start_pos);
  get_lowest_score_path(grid, &start_pos, Direction::East, 0)
}

pub fn part_2(grid: &Vec<Vec<char>>) -> i64 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES1: [&str; 15] = [
    "###############",
    "#.......#....E#",
    "#.#.###.#.###.#",
    "#.....#.#...#.#",
    "#.###.#####.#.#",
    "#.#.#.......#.#",
    "#.#.#####.###.#",
    "#...........#.#",
    "###.#.#####.#.#",
    "#...#.....#.#.#",
    "#.#.#.###.#.#.#",
    "#.....#...#.#.#",
    "#.###.#.#.#.#.#",
    "#S..#.....#...#",
    "###############",
  ];
  static TEST_LINES2: [&str; 17] = [
    "#################",
    "#...#...#...#..E#",
    "#.#.#.#.#.#.#.#.#",
    "#.#.#.#...#...#.#",
    "#.#.#.#.###.#.#.#",
    "#...#.#.#.....#.#",
    "#.#.#.#.#.#####.#",
    "#.#...#.#.#.....#",
    "#.#.#####.#.###.#",
    "#.#.#.......#...#",
    "#.#.###.#####.###",
    "#.#.#...#.....#.#",
    "#.#.#.#####.###.#",
    "#.#.#.........#.#",
    "#.#.#.#########.#",
    "#S#.............#",
    "#################",
  ];

  #[test]
  fn part_1_works_with_test_input1() {
    let lines = TEST_LINES1.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 7036);
  }

  #[test]
  fn part_1_works_with_test_input2() {
    let lines = TEST_LINES2.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 11048);
  }

  // #[test]
  // fn part_2_works() {
  //   let lines = TEST_LINES1.iter().map(|x| x.to_string()).collect();
  //   let test_input = parse_input(lines);
  //   assert_eq!(part_2(&test_input), 123);
  // }
}
