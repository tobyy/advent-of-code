use std::{fs, io};

fn load_lines_from_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let value = fs::read_to_string(file_path)?;
    Ok(load_lines_from_str(&value))
}

pub fn load_lines_from_str(value: &str) -> Vec<String> {
    value.lines().map(String::from).collect()
}

fn read_first(line: &String, value_for_token: &[(&str, u32)]) -> Option<u32> {
    for i in 0..line.len() {
        for (s,r) in value_for_token.iter() {
            if line[i..].starts_with(s){
                return Some(*r);
            }
        }
    }
    None
}

fn read_last(line: &String, value_for_token: &[(&str, u32)]) -> Option<u32> {
    for i in 0..line.len() {
        for (s,r) in value_for_token {
            let end_index = line.len() - i;
            if line[..end_index].ends_with(s){
                return Some(*r);
            }
        }
    }
    None
}

fn parse_line(line: &String, value_for_token: &[(&str, u32)]) -> Option<u32> {
    match (read_first(line, value_for_token), read_last(line, value_for_token)) {
        (Some(first), Some(last)) => Some(first * 10 + last),
        _ => None
    }
}


pub fn part1(lines: &[String]) -> String {
    let mut sum: u32 = 0;
    let value_for_token = vec![("0", 0), ("1", 1) , ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9)];
    for line in lines {
        sum += parse_line(line, &value_for_token).expect("Should have been able to read star id") 
    }
    format!("{}", sum)
}

pub fn part2(lines: &[String]) -> String {
    let mut sum: u32 = 0;
    let value_for_token = vec![("0", 0), ("1", 1) , ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9), ("one", 1),("two", 2), ("three",3), ("four", 4), ("five", 5), ("six",6), ("seven",7), ("eight",8), ("nine",9)];
    for line in lines {
        sum += parse_line(line, &value_for_token).expect("Should have been able to read star id") 
    }
    format!("{}", sum)
}

pub fn run() {
    let part1_file_path = "data/day01/part1.txt";
    let part2_file_path = "data/day01/part2.txt";
    println!("Result part 1 = {}", part1(&load_lines_from_file(part1_file_path).unwrap()));
    println!("Result part 2 = {}", part2(&load_lines_from_file(part2_file_path).unwrap()));
}

fn main() {
    run();
}


#[cfg(test)]
mod tests {

    use crate::{load_lines_from_str, part1, part2};

    #[test]
    fn test_part1() {
        let test_val = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", part1(&load_lines_from_str(test_val)));
    }


    #[test]
    fn test_part2() {
        let test_val = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", part2(&load_lines_from_str(test_val)));
    }
}
