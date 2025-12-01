use std::fs::File;
use cairo::{ ImageSurface, Format, Context };

pub fn part_1(robots: &Vec<((i64, i64), (i64, i64))>, m: i64, n: i64) -> i64 {
  let middle_x = n / 2;
  let middle_y = m / 2;
  let mut quadrants = vec![0_i64; 4];
  for (robot_p, robot_v) in robots {
    let mut x = robot_p.0;
    let mut y = robot_p.1;
    for _ in 0..100 {
      let mut next_x = x + robot_v.0;
      let mut next_y = y + robot_v.1;
      if next_x >= n {
        next_x = next_x - n;
      } else if next_x < 0 {
        next_x = next_x + n;
      }
      if next_y >= m {
        next_y = next_y - m;
      } else if next_y < 0 {
        next_y = next_y + m;
      }
      x = next_x;
      y = next_y;
    }
    let mut quad_idx = None;
    if x < middle_x && y < middle_y {
      quad_idx = Some(0);
    } else if x < middle_x && y > middle_y {
      quad_idx = Some(1);
    } else if x > middle_x && y < middle_y {
      quad_idx = Some(2);
    } else if x > middle_x && y > middle_y {
      quad_idx = Some(3);
    }
    if let Some(quad_idx) = quad_idx {
      quadrants[quad_idx] += 1;
    }
  }
  quadrants.iter().fold(1, |acc, x| acc * x)
}

pub fn part_2(robots: &Vec<((i64, i64), (i64, i64))>, m: i64, n: i64) -> i64 {
  let mut robots = robots.clone();
  const FONT_SIZE: i32 = 13;
  const PADDING: i32 = 28;
  for k in 1..=101*103 {
    let mut grid = vec![vec![' '; m as usize]; n as usize];
    for idx in 0..robots.len() {
      let (mut robot_p, robot_v) = robots[idx];
      let mut next_x = robot_p.0 + robot_v.0;
      let mut next_y = robot_p.1 + robot_v.1;
      if next_x >= n {
        next_x = next_x - n;
      } else if next_x < 0 {
        next_x = next_x + n;
      }
      if next_y >= m {
        next_y = next_y - m;
      } else if next_y < 0 {
        next_y = next_y + m;
      }
      robot_p.0 = next_x;
      robot_p.1 = next_y;
      robots[idx].0 = robot_p;
      grid[robot_p.0 as usize][robot_p.1 as usize] = 'X';
    }
    if k % 101 != 111 % 101 {
      continue;
    }
    let grid_text = grid.iter().map(|x| x.iter().collect::<String>()).collect::<Vec<String>>();
    let ext = "png";
    let mut output_file = File::create(format!("./packages/day-14/images/{:0>6}.{}", k, ext))
      .expect("Unable to create PNG file");
    let surface = ImageSurface::create(
        Format::ARgb32,
        n  as i32 * 8,
        m as i32 * 14,
      )
      .expect("Couldn't create a surface!");
    let context = Context::new(&surface).expect("Couldn't create a context!");
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.select_font_face(
      "monospace",
      cairo::FontSlant::Normal,
      cairo::FontWeight::Bold
    );
    context.set_font_size(FONT_SIZE as f64);
    context.move_to(0.0, PADDING as f64);
    context.show_text(&format!("Seconds: {k}")).expect("Couldn't paint!");
    context.set_font_size(FONT_SIZE as f64);
    for (i, line) in grid_text.iter().enumerate() {
      context.move_to(0.0 as f64, ((i + 1) as f64 * FONT_SIZE as f64) + PADDING as f64);
      context.show_text(line).expect("Couldn't paint!");
    }
    context.set_font_size(FONT_SIZE as f64);
    context.move_to(0.0, ((n + 1) as f64 * FONT_SIZE as f64) + (FONT_SIZE * 2) as f64 + PADDING as f64);
    context.show_text(&format!("Seconds: {k}")).expect("Couldn't paint!");
    surface.write_to_png(&mut output_file)
      .expect("Couldn't write to png");
  }
  0
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 12] = [
    "p=0,4 v=3,-3",
    "p=6,3 v=-1,-3",
    "p=10,3 v=-1,2",
    "p=2,0 v=2,-1",
    "p=0,0 v=1,3",
    "p=3,0 v=-2,-2",
    "p=7,6 v=-1,-3",
    "p=3,0 v=-1,-2",
    "p=9,3 v=2,3",
    "p=7,3 v=-1,2",
    "p=2,4 v=2,-3",
    "p=9,5 v=-3,-3",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input, 11, 7), 12);
  }
}
