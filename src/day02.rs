use std::{fs, cmp, io};
use regex::Regex;


fn load_lines_from_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let value = fs::read_to_string(file_path)?;
    Ok(load_lines_from_str(&value))
}

pub fn load_lines_from_str(value: &str) -> Vec<String> {
    value.lines().map(String::from).collect()
}

pub struct Round {
    r: usize,
    g: usize,
    b: usize,
}

pub struct RegexParsers {
    re_cubeset_named : Regex,
    re_line : Regex,
}

pub fn gen_re_parsers() -> RegexParsers {
    let re_str_cubeset: &str = "([0-9]+)\\s*(red|blue|green)";
    let re_str_cubeset_named: &str = "(?P<count>[0-9]+)\\s*(?P<color>red|blue|green)";
    let re_str_game: &str = "Game\\s*(?P<game>[0-9]+)";
    let re_str_round = format!("({})\\s*(,\\s*{})*", re_str_cubeset, re_str_cubeset);
    let re_str_line = format!("^\\s*{}\\s*\\:\\s*(?P<rounds>{}\\s*(\\;\\s*{})*)\\s*$", re_str_game, re_str_round, re_str_round);
    RegexParsers{re_line: Regex::new(&re_str_line).unwrap(),re_cubeset_named: Regex::new(re_str_cubeset_named).unwrap()}
}

fn parse_line(line: &str, re_parsers: &RegexParsers) -> (usize, Vec<Round>) {
    let caps = re_parsers.re_line.captures(line).unwrap();
    let game = caps["game"].parse::<usize>().unwrap();
    let rounds = caps["rounds"].split(';');

    let mut grab_rounds = vec![];
    for round in rounds {
        let (mut r, mut g, mut b) = (0, 0, 0);
        for cubeset_str in round.split(',') {
            let cubeset = re_parsers.re_cubeset_named.captures(cubeset_str).unwrap();
            let count = cubeset["count"].parse::<usize>().unwrap();
            match &cubeset["color"] {
                "red" => {r = count},
                "green" => {g = count},
                "blue" => {b = count},
                _ => {}
            };
        }
        grab_rounds.push(Round{r,g,b});
    }
    (game, grab_rounds) 
}


pub fn part2(lines: &[String]) -> String {
    let re_parsers = gen_re_parsers(); 
    let mut sum = 0;
    for line in lines {
        let (_game, rounds) = parse_line(line, &re_parsers);
        let (mut r, mut g, mut b) = (0, 0, 0);
        for round in rounds {
            r= cmp::max(round.r, r);
            g= cmp::max(round.g, g);
            b= cmp::max(round.b, b);
        }
        sum += r*g*b;
    }
    format!("{}", sum)
}


pub fn part1(lines: &[String], restriction: &Round)-> String {
    let re_parsers = gen_re_parsers(); 
    let mut sum = 0;
    for line in lines {
        let (game, rounds) = parse_line(line, &re_parsers);
        let mut invalid = false;
        for round in rounds.iter() {
            if !(round.r <= restriction.r && round.g <= restriction.g && round.b <= restriction.b) {
                invalid = true;
                break;
            }
        }
        if !invalid {
            sum += game;
        }
    }
    format!("{}", sum)
}

pub fn run() {
    let part1_file_path = "data/day02/part1.txt";
    let part2_file_path = "data/day02/part2.txt";
    let part1_bag = Round{r:12, g:13, b:14};
    println!("Result part 1 = {}", part1(&load_lines_from_file(part1_file_path).unwrap(), &part1_bag));
    println!("Result part 2 = {}", part2(&load_lines_from_file(part2_file_path).unwrap()));
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let test_val = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let part1_bag = Round{r:12, g:13, b:14};
        let exp_result = "8";
        assert_eq!(exp_result, part1(&load_lines_from_str(test_val), &part1_bag));
    }


    #[test]

    fn test_part2() {
        let test_val = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let exp_result = "2286";
        assert_eq!(exp_result, part2(&load_lines_from_str(test_val)));
    }
}
