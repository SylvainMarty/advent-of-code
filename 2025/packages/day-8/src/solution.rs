use std::collections::HashSet;
use crate::types::Point;
use crate::functions::get_points_by_distance_asc;

pub fn part_1(lines: &Vec<Point>, max_connections: usize) -> i64 {
  let distances = get_points_by_distance_asc(lines);

  let mut circuits: Vec<HashSet<Point>> = Vec::new();
  let mut remaining_connections = max_connections;
  let mut i = 0;
  while remaining_connections > 0 && i < distances.len() {
    let (_, (point1, point2)) = distances[i];
    let mut found = Some(vec![]);
    for j in 0..circuits.len() {
      let circuit = &mut circuits[j];
      if circuit.contains(&point1) && circuit.contains(&point2) {
        found = None; // We don't count 2 points in the same circuit
        break;
      }
      if circuit.contains(&point1) {
        circuit.insert(point2);
        found.as_mut().unwrap().push(j);
        continue;
      }
      if circuit.contains(&point2) {
        circuit.insert(point1);
        found.as_mut().unwrap().push(j);
        continue;
      }
    }
    if found.is_some() {
      let found = found.unwrap();
      if found.is_empty() {
        circuits.push(HashSet::from([point1, point2]));
      } else if found.len() > 1 {
        let circuit2 = &circuits[found[1]].clone();
        let circuit1 = &mut circuits[found[0]];
        circuit1.extend(circuit2.iter().cloned());
        circuits.remove(found[1]);
      }
    }
    remaining_connections -= 1;
    i += 1;
  }

  let mut biggest_circuits: Vec<usize> = circuits.iter().map(|x| x.len()).collect();
  biggest_circuits.sort_by(|a, b| b.cmp(a)); // sort descending to get biggest first
  biggest_circuits.iter().take(3).map(|&x| x as i64).product()
}

pub fn part_2(lines: &Vec<Point>) -> i64 {
  let distances = get_points_by_distance_asc(lines);

  let mut circuits: Vec<HashSet<Point>> = Vec::new();
  let mut i = 0;
  let mut last_points = distances[i].1;
  // Run the loop until its one big circuit containing all points
  while circuits.len() != 1 || circuits.first().unwrap().len() != lines.len() {
    let (_, (point1, point2)) = distances[i];
    let mut found = Some(vec![]);
    for j in 0..circuits.len() {
      let circuit = &mut circuits[j];
      if circuit.contains(&point1) && circuit.contains(&point2) {
        break;
      }
      if circuit.contains(&point1) {
        circuit.insert(point2);
        found.as_mut().unwrap().push(j);
        continue;
      }
      if circuit.contains(&point2) {
        circuit.insert(point1);
        found.as_mut().unwrap().push(j);
        continue;
      }
    }
    if found.is_some() {
      let found = found.unwrap();
      if found.is_empty() {
        circuits.push(HashSet::from([point1, point2]));
      } else if found.len() > 1 {
        // Merge the two circuits into one
        let circuit2 = &circuits[found[1]].clone();
        let circuit1 = &mut circuits[found[0]];
        circuit1.extend(circuit2.iter().cloned());
        circuits.remove(found[1]);
      }
    }
    last_points = (point1, point2);
    if i+1 == distances.len() {
      i = 0;
    } else {
      i += 1;
    }
  }

  last_points.0.0 * last_points.1.0
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::input::parse_input;

  static TEST_LINES: [&str; 20] = [
    "162,817,812",
    "57,618,57",
    "906,360,560",
    "592,479,940",
    "352,342,300",
    "466,668,158",
    "542,29,236",
    "431,825,988",
    "739,650,466",
    "52,470,668",
    "216,146,977",
    "819,987,18",
    "117,168,530",
    "805,96,715",
    "346,949,466",
    "970,615,88",
    "941,993,340",
    "862,61,35",
    "984,92,344",
    "425,690,689",
  ];

  #[test]
  fn part_1_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_1(&test_input, 10), 40);
  }

  #[test]
  fn part_2_works() {
    let lines = TEST_LINES.iter().map(|x| x.to_string()).collect();
    let test_input = parse_input(lines);
    assert_eq!(part_2(&test_input), 25272);
  }
}
