use std::collections::HashSet;
use crate::types::Point;

pub fn euclidean_squared_distance(p1: Point, p2: Point) -> f64 {
  ((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)) as f64
}

pub fn get_points_by_distance_asc(points: &Vec<Point>) -> Vec<(f64, (Point, Point))> {
  let mut distances: Vec<(f64, (Point, Point))> = Vec::new();
  let mut distances_set = HashSet::new();
  let n = points.len();
  for i in 0..n {
    let (x, y, z) = points[i];
    for j in 0..n {
      if i == j
      || distances_set.contains(&(points[i], points[j]))
      || distances_set.contains(&(points[j], points[i])) {
        continue;
      }
      let (x2, y2, z2) = points[j];
      let distance = euclidean_squared_distance((x, y, z), (x2, y2, z2));
      distances.push((distance, (points[i], points[j])));
      distances_set.insert((points[i], points[j]));
      distances_set.insert((points[j], points[i]));
    }
  }
  distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
  distances
}
