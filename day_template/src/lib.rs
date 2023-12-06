use regex::Regex;

pub fn part1(_lines:Vec<String>)->i32 {
  let mut result:i32 = 0;

  for line in lines {
    if !line.is_empty() {
      // Index: 0 game, 1 red, 2 green, 3 blue

    } 
  }


  return result;
}
pub fn part2(lines:Vec<String>)->i32 {
  let mut result:i32 = 0;

  for line in lines {
    if !line.is_empty() {
      // Index: 0 game, 1 red, 2 green, 3 blue

    } 
  }

  return result;
}

#[cfg(test)]
mod tests {
  use crate::part1;
  //use crate::part2;

  use shared::read_lines;

  #[test]
  fn test1_part1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 8);
  }

  /*#[test]
  fn test1_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 2439);
  }

  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 2286);
  }

  #[test]
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 63711);
  }*/
  
}