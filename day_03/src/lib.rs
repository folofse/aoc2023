use regex::{Regex, Captures, Match};
use std::collections::HashMap;
use std::cmp;

pub fn part1(lines:Vec<String>)->i32 {
  let mut result:i32 = 0;
  let hash_lines = lines.clone();
  let hash:Vec<Vec<char>> = hash_lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect();
  let re = Regex::new(r"(\d*)").unwrap();

  for (pos_y, line) in lines.iter().enumerate() {
    let line_caps:Vec<Captures> = re.captures_iter(line.as_str()).collect();
    for cap in line_caps {
      let pos_x = cap.get(0).unwrap();

      if is_engine_part(pos_x, pos_y as i32, &hash) {
        result += pos_x.as_str().parse::<i32>().unwrap();
      } 
    }
  }

  return result;
}

pub fn is_engine_part(pos_x:Match, pos_y:i32, hash:&Vec<Vec<char>> )->bool{
  if pos_x.as_str() == "" {
    return false;
  }

  let re = Regex::new(r"\d|\.").unwrap();
  let start_x:i32 = cmp::max(pos_x.start() as i32 - 1, 0);
  let end_x:i32 = cmp::min(pos_x.end() as i32 + 1, (hash[0].len() -1 ) as i32);
  let start_y:i32 = cmp::max(pos_y - 1, 0);
  let end_y:i32 = cmp::min(pos_y + 1, (hash.len() -1) as i32);

  for y in start_y..end_y+1 {
    for x in start_x..end_x {
      if !re.is_match(hash[y as usize][x as usize].to_string().as_str()) {
        return true;
      }
    }
  }

  return false
}
pub fn part2(lines:Vec<String>)->i32 {
  let mut result:i32 = 0;
  let mut gears_map:HashMap<String, Vec<i32>> = HashMap::new();

  let hash_lines = lines.clone();
  let hash:Vec<Vec<char>> = hash_lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect();
  let re = Regex::new(r"(\d*)").unwrap();

  for (pos_y, line) in lines.iter().enumerate() {
    let line_caps:Vec<Captures> = re.captures_iter(line.as_str()).collect();
    for cap in line_caps {
      let pos_x = cap.get(0).unwrap();
      populate_gear_map(pos_x, pos_y as i32, &hash, &mut gears_map);
    }
  }

  for (_key, gear) in gears_map.iter() {
    if gear.len() == 2{
      result += gear[0] * gear[1];
    }
  }

  return result;
}

pub fn populate_gear_map(pos_x:Match, pos_y:i32, hash:&Vec<Vec<char>>, gears_map:&mut HashMap<String, Vec<i32>> ){
  if pos_x.as_str() == "" {
    return;
  }

  let re = Regex::new(r"\*").unwrap();
  let start_x:i32 = cmp::max(pos_x.start() as i32 - 1, 0);
  let end_x:i32 = cmp::min(pos_x.end() as i32 + 1, (hash[0].len() -1 ) as i32);
  let start_y:i32 = cmp::max(pos_y - 1, 0);
  let end_y:i32 = cmp::min(pos_y + 1, (hash.len() -1) as i32);

  for y in start_y..end_y+1 {
    for x in start_x..end_x {
      if re.is_match(hash[y as usize][x as usize].to_string().as_str()) {
        let mut key = "".to_string();
        key.push_str(y.to_string().as_str());
        key.push_str(",");
        key.push_str(x.to_string().as_str());


        println!("Key {:?}", key);
        let value = pos_x.as_str().parse::<i32>().unwrap();

        if !gears_map.contains_key(key.as_str()){
          gears_map.insert(key.to_string(), vec![value]);
        }else{
          let gears = gears_map.get_mut(key.as_str()).unwrap();
          gears.push(value);
        }
      }
    }
  }
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

    assert_eq!(part1(lines), 4361);
  }

  #[test]
  fn test1_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part1(lines), 550934);
  }
  

  #[test]
  fn test2_part1() {
    let lines = read_lines("./src/example.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 467835);
  }

  #[test]
  fn test2_part2() {
    let lines = read_lines("./src/input.txt")
      .iter()
      .map(|f| f.parse().unwrap())
      .collect();

    assert_eq!(part2(lines), 81997870);
  }
}