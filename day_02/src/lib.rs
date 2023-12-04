use regex::Regex;

pub fn part1(lines:Vec<String>)->i32 {

  return 1
}

pub fn part2(lines:Vec<String>)->i32 {
  
  return 1
}


#[cfg(test)]
mod tests {
  use crate::part1;
  use crate::part2;

  use shared::read_lines;

  #[test]
  fn test1_part1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 142);
  }

  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 55488);
  }

  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 281);
  }

  #[test]
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 5560123);
  }
}