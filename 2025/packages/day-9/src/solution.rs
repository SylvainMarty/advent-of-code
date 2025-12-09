use crate::{functions::{is_tile_in_polygon, segments_cross}, gen_image::draw_polygon_to_file, types::Point};

pub fn part_1(points: &Vec<Point>) -> i64 {
  let n = points.len();
  let mut largest_area = 0;
  for i in 0..n {
    let (x1, y1) = points[i];
    for j in i+1..n {
      if i == j {
        continue;
      }
      let (x2, y2) = points[j];
      let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
      if area > largest_area {
        largest_area = area;
      }
    }
  }
  largest_area
}

pub fn part_2(polygon: &Vec<Point>) -> i64 {
  let n = polygon.len();
  let mut largest_area = 0;
  let mut largest_tile: [Point; 4] = [(0, 0), (0, 0), (0, 0), (0, 0)];

  // Create a vector of all the links (pairs of indices and their squared distance)
  let mut links: Vec<((usize, usize), i64)> = Vec::new();
  for i in 0..n {
    let (x1, y1) = polygon[i];
    let j = if i+1 == n { 0 } else { i+1 };
    let (x2, y2) = polygon[j];
    let distance = (x2 - x1).pow(2) + (y2 - y1).pow(2);
    links.push(((i, j), distance));
  }
  // Sort descending by distance (longest to shortest)
  links.sort_by(|a, b| b.1.cmp(&a.1));

  for i in 0..n {
    let (x1, y1) = polygon[i];
    for j in i+1..n {
      if i == j {
        continue;
      }
      let (x2, y2) = polygon[j];
      let (x3, y3) = (x2, y1);
      let (x4, y4) = (x1, y2);
      // We check if the rectangle is inside the polygon
      if !is_tile_in_polygon((x3, y3), polygon, n) || !is_tile_in_polygon((x4, y4), polygon, n) {
        continue;
      }
      // Check if any rectangle edge crosses a polygon edge (collinear is allowed)
      let mut does_cross_polygon_edge = false;
      for ((pi, pj), _) in &links {
        let edge_start = polygon[*pi];
        let edge_end = polygon[*pj];
        if segments_cross((x1, y1), (x3, y3), edge_start, edge_end) ||
           segments_cross((x3, y3), (x2, y2), edge_start, edge_end) ||
           segments_cross((x2, y2), (x4, y4), edge_start, edge_end) ||
           segments_cross((x4, y4), (x1, y1), edge_start, edge_end) {
          does_cross_polygon_edge = true;
          break;
        }
      }
      if does_cross_polygon_edge {
        continue;
      }
      let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
      if area > largest_area {
        largest_area = area;
        largest_tile = [(x1, y1), (x2, y2), (x3, y3), (x4, y4)];
      }
    }
  }
  draw_polygon_to_file(polygon, largest_tile, "polygon.bmp").unwrap();
  largest_area
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 8] = [
    "7,1",
    "11,1",
    "11,7",
    "9,7",
    "9,5",
    "2,5",
    "2,3",
    "7,3",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input), 50);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_2(&test_input), 24);
  }
}
