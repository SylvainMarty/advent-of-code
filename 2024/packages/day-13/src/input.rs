use utils::filesystem::read_lines;

pub fn get_input(filename: &str) -> Vec<((f64, f64), (f64, f64), (f64, f64))> {
  let res = read_lines(format!("./packages/day-13/src/{}.txt", filename));
  match res {
    Ok(lines) => {
      let lines = lines.map_while(Result::ok);
      parse_input(lines.collect())
    }
    Err(e) => panic!("Error reading file: {}", e),
  }
}

pub fn parse_input(lines: Vec<String>) -> Vec<((f64, f64), (f64, f64), (f64, f64))> {
  let mut vec = Vec::new();
  let mut button_a: (f64, f64) = (0.0, 0.0);
  let mut button_b: (f64, f64) = (0.0, 0.0);
  let mut price: (f64, f64) = (0.0, 0.0);
  for line in lines {
    if line.is_empty() {
      vec.push((button_a, button_b, price));
      button_a = (0.0, 0.0);
      button_b = (0.0, 0.0);
      price = (0.0, 0.0);
      continue;
    }
    let btn_split = line.split(": ").collect::<Vec<&str>>();
    match btn_split[0] {
      "Button A" => {
        let x_and_y_split = btn_split[1].split(", ").collect::<Vec<&str>>();
        button_a = (
          x_and_y_split[0].split("+").collect::<Vec<&str>>().last().unwrap().parse::<f64>().unwrap(),
          x_and_y_split[1].split("+").collect::<Vec<&str>>().last().unwrap().parse::<f64>().unwrap(),
        );
      }
      "Button B" => {
        let x_and_y_split = btn_split[1].split(", ").collect::<Vec<&str>>();
        button_b = (
          x_and_y_split[0].split("+").collect::<Vec<&str>>().last().unwrap().parse::<f64>().unwrap(),
          x_and_y_split[1].split("+").collect::<Vec<&str>>().last().unwrap().parse::<f64>().unwrap(),
        );
      }
      "Prize" => {
        let x_and_y_split = btn_split[1].split(", ").collect::<Vec<&str>>();
        price = (
          x_and_y_split[0].split("=").collect::<Vec<&str>>().last().unwrap().parse::<f64>().unwrap(),
          x_and_y_split[1].split("=").collect::<Vec<&str>>().last().unwrap().parse::<f64>().unwrap(),
        );
      }
      _ => {
        println!("Cannot parse line: {:#?}", line);
        panic!("Cannot parse line!");
      },
    }
  }
  vec.push((button_a, button_b, price));
  vec
}
