use std::collections::HashSet;

use crate::functions::{extract_rules_and_updates, is_update_valid};

pub fn part_1(lines: Vec<String>) -> i32 {
  let (inverted_rules, updates) = extract_rules_and_updates(lines);
  let mut result = 0;
  let mut error_updates_count = 0;
  for update in updates {
    let middle_value = update[update.len() / 2];
    if !is_update_valid(&update, &inverted_rules) {
      error_updates_count += 1;
      continue;
    }
    result += middle_value;
  }
  println!("Errors updates count: {}", error_updates_count);
  result
}

pub fn part_2(lines: Vec<String>) -> i32 {
  let (inverted_rules, updates) = extract_rules_and_updates(lines);
  let mut result = 0;
  for update in updates {
    if is_update_valid(&update, &inverted_rules) {
      continue;
    }
    let update_set: HashSet<&i32> = update.iter().collect::<HashSet<&i32>>();
    let mut fixed_update = update.clone();
    let l = fixed_update.len();
    let mut num_seen = HashSet::new();
    let mut idx = 0;
    while idx < l {
      let num: i32;
      num = fixed_update[idx];
      if !inverted_rules.contains_key(&num) {
        // Line is valid: no rule for this number
        num_seen.insert(num.clone());
        idx += 1;
        continue;
      }
      let mut has_error = false;
      for arrive_after_num in inverted_rules.get(&num).unwrap() {
        if !update_set.contains(arrive_after_num) || num_seen.contains(arrive_after_num) {
          // Num position is valid
          continue;
        }
        has_error = true;
        break;
      }
      if !has_error {
        num_seen.insert(num.clone());
        idx += 1;
        continue;
      }
      // We remove the invalid number
      fixed_update.remove(idx);
      // and insert it at the end of the fixed update
      fixed_update.push(num.clone());
    }
    let middle_value = fixed_update[fixed_update.len() / 2];
    result += middle_value;
  }
  result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LINES: [&str; 28] = [
      "47|53",
      "97|13",
      "97|61",
      "97|47",
      "75|29",
      "61|13",
      "75|53",
      "29|13",
      "97|29",
      "53|29",
      "61|53",
      "97|53",
      "61|29",
      "47|13",
      "75|47",
      "97|75",
      "47|61",
      "75|61",
      "47|29",
      "75|13",
      "53|13",
      "",
      "75,47,61,53,29",
      "97,61,53,29,13",
      "75,29,13",
      "75,97,47,61,53",
      "61,13,29",
      "97,13,75,29,47",
    ];

    #[test]
    fn part_1_works() {
      assert_eq!(part_1(TEST_LINES.into_iter().map(|x| x.to_string()).collect()), 143);
    }

    #[test]
    fn part_2_works() {
      assert_eq!(part_2(TEST_LINES.into_iter().map(|x| x.to_string()).collect()), 123);
    }
}
