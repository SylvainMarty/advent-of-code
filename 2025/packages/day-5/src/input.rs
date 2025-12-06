use utils::filesystem::read_lines;
use std::collections::BTreeMap;

pub fn get_input(filename: &str) -> (BTreeMap<i64, i64>, Vec<i64>) {
  let res = read_lines(format!("./packages/day-5/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.map_while(Result::ok);
      parse_input(lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input(lines: Vec<String>) -> (BTreeMap<i64, i64>, Vec<i64>) {
  let mut id_ranges = BTreeMap::new();
  let mut ids = Vec::new();
  let mut id_ranges_done = false;
  for line in lines {
    if line == "" {
      id_ranges_done = true;
      continue;
    }
    if !id_ranges_done {
      let mut split = line.split('-');
      let start = split.next().unwrap().parse::<i64>().unwrap();
      let end = split.next().unwrap().parse::<i64>().unwrap();
      // We merge overlapping start ranges by taking the max of the end values
      id_ranges.insert(start, *id_ranges.get(&start).unwrap_or(&end).max(&end));
    } else {
      ids.push(line.parse::<i64>().unwrap());
    }
  }
  (id_ranges, ids)
}
