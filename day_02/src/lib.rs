use regex::Regex;

pub fn part1(lines:Vec<String>)->i32 {
  let mut result:i32 = 0;
  let max_reds:i32 = 12;
  let max_green:i32 = 13;
  let max_blue:i32 = 14;
  
  for line in lines {
    if !line.is_empty() {

      // Index: 0 game, 1 red, 2 green, 3 blue
      let cubes = get_cubes(line);

      if cubes.get(1).unwrap().iter().max().unwrap() <= &max_reds &&
         cubes.get(2).unwrap().iter().max().unwrap() <= &max_green &&
         cubes.get(3).unwrap().iter().max().unwrap() <= &max_blue{
        
        result += cubes.get(0).unwrap().get(0).unwrap();
      }
    } 
  }

  return result;
}
pub fn part2(lines:Vec<String>)->i32 {
  let mut result:i32 = 0;

  for line in lines {
    if !line.is_empty() {
      // Index: 0 game, 1 red, 2 green, 3 blue
      let cubes = get_cubes(line);

      let min_red = cubes.get(1).unwrap().iter().max().unwrap();
      let min_green = cubes.get(2).unwrap().iter().max().unwrap();
      let min_blue = cubes.get(3).unwrap().iter().max().unwrap();

      result += min_red * min_green * min_blue;
    } 
  }

  return result;
}

pub fn get_cubes(line:String)->Vec<Vec<i32>> {
  let re_game = Regex::new(r"Game (\d*):").unwrap();
  let re_red = Regex::new(r"(\d*) red").unwrap();
  let re_green = Regex::new(r"(\d*) green").unwrap();
  let re_blue = Regex::new(r"(\d*) blue").unwrap();

  let game_caps:Vec<i32> = re_game.captures_iter(line.as_str()).map(|cap | 
    cap.get(1).unwrap().as_str().parse::<i32>().unwrap()
  ).collect();
  let red_caps:Vec<i32> = re_red.captures_iter(line.as_str()).map(|cap | 
    cap.get(1).unwrap().as_str().parse::<i32>().unwrap()
  ).collect();
  let green_caps:Vec<i32> = re_green.captures_iter(line.as_str()).map(|cap | 
    cap.get(1).unwrap().as_str().parse::<i32>().unwrap()
  ).collect();
  let blue_caps:Vec<i32> = re_blue.captures_iter(line.as_str()).map(|cap | 
    cap.get(1).unwrap().as_str().parse::<i32>().unwrap()
  ).collect();

  return vec![game_caps.clone(), red_caps.clone(), green_caps.clone(), blue_caps.clone() ];
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

    assert_eq!(part1(lines), 8);
  }

  #[test]
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
  }
  
}