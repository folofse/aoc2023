use regex::Regex;

pub fn part1(lines:Vec<String>)->i32 {
  let mut result_val:i32 = 0;
  let re = Regex::new(r"[0-9]").unwrap();

  for line in lines {
    if !line.is_empty() {
      let matches:Vec<&str> = re.find_iter(&line).map(|mat| mat.as_str()).collect();
      let mut line_val:String = "".to_string();

      line_val.push_str(matches.first().unwrap());
      line_val.push_str(matches.last().unwrap());

      result_val += line_val.parse::<i32>().unwrap();
    } 
  }

  return result_val
}

pub fn part2(lines:Vec<String>)->i32 {
  let mut result_val:i32 = 0;
  let to_match = vec!["1","2","3","4","5","6","7","8","9","one","two","three","four","five","six","seven","eight","nine"];

  for line in lines {
 
    if !line.is_empty() {
      let mut start_pos = line.len();
      let mut start_val = "".to_string();
      let mut end_pos= 0;
      let mut end_val= "".to_string();

      let mut line_val:String = "".to_string();

      for c in to_match.iter() {
        let matches:Vec<_> = line.match_indices(c).collect();

        if matches.first().is_some() {
          let match_start = matches.first().unwrap();
          if match_start.0 < start_pos {
            start_pos = match_start.0;
            start_val = get_item_val(match_start.1);
          }
        }

        if matches.last().is_some() {
          let match_end = matches.last().unwrap();
          if match_end.0 > end_pos {
            end_pos = match_end.0;
            end_val = get_item_val(match_end.1);
          }
        }
      }
      
      line_val.push_str(start_val.as_str());
      
      if !end_val.is_empty(){
        line_val.push_str(end_val.as_str());
      }else{
        line_val.push_str(start_val.as_str());
      }
      
      result_val += line_val.parse::<i32>().unwrap();

    } 
  }

  return result_val
}


pub fn get_item_val(item: &str)->String{
  let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

  let index = numbers.iter().position(|&n| n == item);

  if index.is_some() {
    return index.unwrap().to_string();
  }else{
    return item.to_string();
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
    let lines = read_lines("./src/example2.txt")
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