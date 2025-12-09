use crate::types::Point;

pub fn is_tile_in_polygon(point: Point, polygon: &Vec<Point>, n: usize) -> bool {
    let (px, py) = point;

    // Check if point is on a vertex
    for &(vx, vy) in polygon {
        if px == vx && py == vy {
            return true;
        }
    }

    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        // Check if point is on this edge
        if point_on_segment((px, py), (xi, yi), (xj, yj)) {
            return true;
        }

        // Standard ray casting
        if (
            (yi > py) != (yj > py))
            && (px < (xj - xi) * (py - yi) / (yj - yi) + xi
        ) {
            inside = !inside;
        }
        j = i;
    }

    inside
}

fn point_on_segment(p: (i64, i64), a: (i64, i64), b: (i64, i64)) -> bool {
    let (px, py) = p;
    let (ax, ay) = a;
    let (bx, by) = b;

    // Check if p is collinear with a-b using cross product
    let cross = cross_product((px, py), (ax, ay), (bx, by));
    if cross != 0 {
        return false;
    }

    let min_x = ax.min(bx);
    let max_x = ax.max(bx);
    let min_y = ay.min(by);
    let max_y = ay.max(by);
    // Check if p is within the bounding box of a-b
    px >= min_x && px <= max_x && py >= min_y && py <= max_y
}

/// Check if segment (a1, a2) crosses segment (b1, b2)
/// Returns true only for proper crossings, NOT for collinear/touching cases
pub fn segments_cross(a1: Point, a2: Point, b1: Point, b2: Point) -> bool {
    let d1 = cross_product(a1, a2, b1);
    let d2 = cross_product(a1, a2, b2);
    let d3 = cross_product(b1, b2, a1);
    let d4 = cross_product(b1, b2, a2);

    // Only return true if points are strictly on opposite sides (proper crossing)
    ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) &&
    ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))
}

/// Cross product of vectors (a→b) and (a→c)
/// Returns: positive if c is left of a→b, negative if right, 0 if collinear
fn cross_product(a: Point, b: Point, c: Point) -> i64 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
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
  fn is_tile_in_polygon_returns_true_for_all_vertices() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let polygon = parse_input(lines);
    let n = polygon.len();
    for p in &polygon {
      assert_eq!(is_tile_in_polygon(*p, &polygon, n), true);
    }
  }

  #[test]
  fn is_tile_in_polygon_returns_false_for_points_outside_polygon() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let polygon = parse_input(lines);
    let n = polygon.len();
    assert_eq!(is_tile_in_polygon((0, 0), &polygon, n), false);
    assert_eq!(is_tile_in_polygon((0, 8), &polygon, n), false);
    assert_eq!(is_tile_in_polygon((12, 0), &polygon, n), false);
    assert_eq!(is_tile_in_polygon((12, 8), &polygon, n), false);
  }

  #[test]
  fn is_tile_in_polygon_returns_true_for_points_inside_polygon() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let polygon = parse_input(lines);
    let n = polygon.len();
    assert_eq!(is_tile_in_polygon((8, 5), &polygon, n), true);
    assert_eq!(is_tile_in_polygon((10, 2), &polygon, n), true);
  }
}
