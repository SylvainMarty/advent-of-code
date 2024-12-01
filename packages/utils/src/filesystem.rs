use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename);
    match file {
      Ok(file) => return Ok(io::BufReader::new(file).lines()),
      Err(e) => return Err(e),
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn it_finds_the_file() {
      assert!(read_lines("./src/filesystem-sample.txt").is_ok());
    }

    #[test]
    fn it_throws_error_when_file_not_found() {
      assert!(read_lines("./src/fs-sample.txt").is_err());
    }

    #[test]
    fn it_reads_lines() {
      let mut counter = 0;
      let res = read_lines("./src/filesystem-sample.txt");
      match res {
        Ok(lines) => {
          for line in lines.flatten() {
            counter += 1;
            assert_eq!(line, counter.to_string());
          }
        }
        Err(e) => panic!("Error reading file: {}", e),
      }
    }
}
