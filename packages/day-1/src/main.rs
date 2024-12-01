use utils::filesystem::read_lines;

fn main() {
  let start = std::time::Instant::now();
  // Part 1
  let (vec1, vec2) = get_input_part1();
  println!("Inputs length: {} {}", vec1.len(), vec2.len());
  let mut sorted_vec1 = vec1;
  sorted_vec1.sort();
  let mut sorted_vec2 = vec2;
  sorted_vec2.sort();
  let mut distance: i32 = 0;
  for i in 0..sorted_vec1.len() {
    println!("Comparing {} and {}", sorted_vec1[i], sorted_vec2[i]);
    distance += (sorted_vec1[i] - sorted_vec2[i]).abs();
  }
  let time_part_1 = start.elapsed();

  // Part 2
  let start_part2 = std::time::Instant::now();
  let mut similarity_score: i32 = 0;
  let (vec1, freq2) = get_input_part2();
  for i in 0..vec1.len() {
    println!(
      "{} has been found {} times in list 2, score is: {}",
      vec1[i],
      freq2[vec1[i] as usize],
      freq2[vec1[i] as usize] * vec1[i]
    );
    similarity_score += vec1[i] * freq2[vec1[i] as usize];
  }
  let time_part_2 = start_part2.elapsed();

  // End
  println!("Distance: {}", distance);
  eprintln!(" > Done part 1 in: {:?}", time_part_1);
  println!("Similarity score: {}", similarity_score);
  eprintln!(" > Done part 2 in: {:?}", time_part_2);
  eprintln!("Total done in: {:?}", start.elapsed()); 
}

fn get_input_part1() -> (Vec<i32>, Vec<i32>) {
  let mut vec1 = Vec::new();
  let mut vec2 = Vec::new();
  let res = read_lines("./packages/day-1/src/input.txt");
  match res {
    Ok(lines) => {
      for line in lines.flatten() {
        let mut split = line.split_whitespace();
        let first_num = split.next();
        match first_num {
          Some(num) => vec1.push(num.parse::<i32>().unwrap()),
          None => panic!("First number not found"),
        }
        let second_num = split.next();
        match second_num {
          Some(num) => vec2.push(num.parse::<i32>().unwrap()),
          None => panic!("Second number not found"),
        }
      }
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
  (vec1, vec2)
}

fn get_input_part2() -> (Vec<i32>, [i32; 99991]) {
  let mut vec1 = Vec::new();
  let mut freq2: [i32; 99991] = [0; 99991];
  let res = read_lines("./packages/day-1/src/input.txt");
  match res {
    Ok(lines) => {
      for line in lines.flatten() {
        let mut split = line.split_whitespace();
        let first_num = split.next();
        match first_num {
          Some(num) => vec1.push(num.parse::<i32>().unwrap()),
          None => panic!("First number not found"),
        }
        let second_num = split.next();
        match second_num {
          Some(num_str) => {
            let num = num_str.parse::<i32>().unwrap();
            freq2[num as usize] += 1;
          },
          None => panic!("Second number not found"),
        }
      }
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
  (vec1, freq2)
}
