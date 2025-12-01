use std::collections::{HashMap, HashSet};

pub fn extract_rules_and_updates(lines: Vec<String>) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
  let mut inverted_rules = HashMap::new();
  let mut updates = Vec::new();
  let mut are_rules_done = false;
  for line in lines {
    if line.is_empty() {
      are_rules_done = true;
      continue;
    }
    if !are_rules_done {
      let numbers = line.split('|').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
      let (left, right) = (numbers[0], numbers[1]);
      if !inverted_rules.contains_key(&right) {
        inverted_rules.insert(right, HashSet::new());
      }
      inverted_rules.get_mut(&right).unwrap().insert(left);
    } else {
      let numbers = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
      updates.push(numbers);
    }
  }
  (inverted_rules, updates)
}

pub fn is_update_valid(update: &Vec<i32>, inverted_rules: &HashMap<i32, HashSet<i32>>) -> bool {
  let update_set = update.iter().collect::<HashSet<&i32>>();
  let mut num_seen = HashSet::new();
  for num in update.clone() {
    if !inverted_rules.contains_key(&num) {
      // Line is valid: no rule for this number
      num_seen.insert(num);
      continue;
    }
    let mut has_error = false;
    for arrive_after_num in inverted_rules.get(&num).unwrap() {
      if !update_set.contains(arrive_after_num) || num_seen.contains(arrive_after_num) {
        // Num position is valid:
        //  - the number that should arrive before is not in the update
        //  - or the number already arrived
        continue;
      }
      has_error = true;
      break;
    }
    if !has_error {
      num_seen.insert(num);
      continue;
    }
    // Line is invalid
    return false;
  }
  true
}
