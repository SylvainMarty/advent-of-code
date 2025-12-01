pub fn turn_dir(turn: &str) -> i64 {
    match turn {
        "L" => -1,
        "R" => 1,
        _ => panic!("Invalid turn: {}", turn),
    }
}
