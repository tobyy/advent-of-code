use std::{fs, io};
use regex::Regex;


fn load_lines_from_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let value = fs::read_to_string(file_path)?;
    Ok(load_lines_from_str(&value))
}

pub fn load_lines_from_str(value: &str) -> Vec<String> {
    value.lines().map(String::from).collect()
}

pub struct RegexParsers {
    re_symbol : Regex,
    re_gear_candidate_symbol : Regex,
    re_number: Regex,
}

pub fn gen_re_parsers() -> RegexParsers {
    let re_str_symbol: &str = "([^0-9\\.])";
    let re_str_gear_candidate_symbol: &str = "(\\*)";
    let re_str_number: &str = "([0-9]+)";
    RegexParsers{re_symbol: Regex::new(re_str_symbol).unwrap(),re_number: Regex::new(re_str_number).unwrap(), re_gear_candidate_symbol: Regex::new(re_str_gear_candidate_symbol).unwrap()}
}

pub struct SymbolPosition {
    symbol: String,
    x: usize,
    y: usize
}

pub struct NumberPosition {
    number: u32,
    x_start: usize,
    x_end: usize,
    y: usize
}

pub fn parse_number_positions(lines: &[String], regex_parsers: &RegexParsers) -> Vec<NumberPosition> {
    let mut number_positions = vec![];
    for  (y, line) in lines.iter().enumerate() {
        for number_match in regex_parsers.re_number.find_iter(line) {
            let x_start = number_match.start();
            let x_end = number_match.end() - 1;
            let number = number_match.as_str().parse::<u32>().unwrap();
            number_positions.push(NumberPosition{number, x_start, x_end, y});
        }
    }
    number_positions
}

pub fn parse_symbol_positions(lines: &[String], regex_parsers: &RegexParsers) -> Vec<SymbolPosition> {
    let mut symbol_positions = vec![];
    for  (y, line) in lines.iter().enumerate() {
        for symbol_match in regex_parsers.re_symbol.find_iter(line) {
            let x = symbol_match.start();
            let symbol = symbol_match.as_str().to_string();
            symbol_positions.push(SymbolPosition{symbol, x,y});
        }
    }
    symbol_positions
}

pub fn parse_gear_candidate_symbol_positions(lines: &[String], regex_parsers: &RegexParsers) -> Vec<SymbolPosition> {
    let mut symbol_positions = vec![];
    for  (y, line) in lines.iter().enumerate() {
        for symbol_match in regex_parsers.re_gear_candidate_symbol.find_iter(line) {
            let x = symbol_match.start();
            let symbol = symbol_match.as_str().to_string();
            symbol_positions.push(SymbolPosition{symbol, x,y});
        }
    }
    symbol_positions
}

pub fn part1(lines: &[String]) -> String {

    let mut sum = 0;
    let re_parsers = gen_re_parsers(); 

    let number_positions = parse_number_positions(lines, &re_parsers);
    let symbol_positions = parse_symbol_positions(lines, &re_parsers);

    for n_p in &number_positions {
        let mut is_part_number = false;
        let x0: usize = if n_p.x_start == usize::MIN {usize::MIN} else {n_p.x_start - 1};
        let y0: usize = if n_p.y == usize::MIN {usize::MIN} else {n_p.y - 1};
        let x1: usize = if n_p.x_end == usize::MAX {usize::MAX} else {n_p.x_end + 1};
        let y1: usize = if n_p.y == usize::MAX {usize::MAX} else {n_p.y + 1};
        for s_p in &symbol_positions {
            if s_p.y >= y0 && s_p.y <= y1 && s_p.x >= x0 && s_p.x <= x1 {
                is_part_number = true;
                break;
            }
        }
        if is_part_number {
            sum += n_p.number;
        }
    }
    format!("{}", sum)
}

pub fn part2(lines: &[String]) -> String {
    let mut sum = 0;
    let re_parsers = gen_re_parsers(); 

    let number_positions = parse_number_positions(lines, &re_parsers);
    let gear_candidate_symbol_positions = parse_gear_candidate_symbol_positions(lines, &re_parsers);

    for g_c_s_p in &gear_candidate_symbol_positions {
        let mut adjacent_numbers = vec![];
        for n_p in &number_positions {
            let x0: usize = if n_p.x_start == usize::MIN {usize::MIN} else {n_p.x_start - 1};
            let y0: usize = if n_p.y == usize::MIN {usize::MIN} else {n_p.y - 1};
            let x1: usize = if n_p.x_end == usize::MAX {usize::MAX} else {n_p.x_end + 1};
            let y1: usize = if n_p.y == usize::MAX {usize::MAX} else {n_p.y + 1};
            if g_c_s_p.y >= y0 && g_c_s_p.y <= y1 && g_c_s_p.x >= x0 && g_c_s_p.x <= x1 {
                adjacent_numbers.push(n_p.number);
            }
        }
        if adjacent_numbers.len() == 2 {
            // gear
            let gear_ratio = adjacent_numbers[0] * adjacent_numbers[1];
            sum += gear_ratio;
        }
    }
    format!("{}", sum)
}



pub fn run() {
    let part1_file_path = "data/day03/part1.txt";
    let part2_file_path = "data/day03/part2.txt";
    println!("Result part 1 = {}", part1(&load_lines_from_file(part1_file_path).unwrap()));
    println!("Result part 2 = {}", part2(&load_lines_from_file(part2_file_path).unwrap()));
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let test_val = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let exp_result = "4361";
        assert_eq!(exp_result, part1(&load_lines_from_str(test_val)));
    }


    #[test]
    fn test_part2() {
        let test_val = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let exp_result = "467835";
        assert_eq!(exp_result, part2(&load_lines_from_str(test_val)));
    }
}
