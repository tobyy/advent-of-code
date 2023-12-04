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
    re_line: Regex,
    re_number: Regex,
}

pub struct Card {
    number: usize,
    match_count: Option<usize>,
    id: usize,
    winning: Vec<usize>,
    own: Vec<usize>
}

pub trait CardPoints {

    fn points(& mut self) -> u32;

    fn matches(& mut self) -> usize;

    fn winning_card_ids(& mut self, max_card_id: usize) -> Vec<usize>;

}

impl CardPoints for Card {
    

    fn matches(& mut self) -> usize {
        if self.match_count.is_none() {
            let mut count: usize = 0;
            for own_num in self.own.iter() {
                if self.winning.contains(own_num) {
                    count += 1;
                }
            }
            self.match_count = Some(count);
        }
        self.match_count.unwrap()
    }

    fn points(& mut self) -> u32 {
        let matches = self.matches();
        if matches == 0 {0} else {2u32.pow(matches as u32 - 1)}
    }

    fn winning_card_ids(& mut self, max_card_id: usize) -> Vec<usize> {
        let mut won_copies = vec![];
        for cid in 0..self.matches() {
            let add_card_id = self.id + 1 + cid;
            if add_card_id < max_card_id {
                won_copies.push(add_card_id);
            }
        }
        won_copies
    }
}


pub fn gen_re_parsers() -> RegexParsers {
    let re_str_line: &str = r"Card\s*(?P<card>[0-9]+)\:(?P<winning>[ 0-9]+)\|(?P<own>[ 0-9]+)";
    let re_str_number: &str = "([0-9]+)";
    RegexParsers{re_line: Regex::new(re_str_line).unwrap(),re_number: Regex::new(re_str_number).unwrap()}
}


pub fn parse_cards(lines: &[String], re_parsers: &RegexParsers) -> Vec<Card> {
    let mut cards = vec![];
    for line in lines {
        let cap_line = re_parsers.re_line.captures(line).unwrap();
        let number = cap_line["card"].parse::<usize>().unwrap();
        let winning = re_parsers.re_number.find_iter(&cap_line["winning"]).map(|w| w.as_str().parse::<usize>().unwrap()).collect();
        let own = re_parsers.re_number.find_iter(&cap_line["own"]).map(|w| w.as_str().parse::<usize>().unwrap()).collect();
        let id  = number - 1;
        cards.push(Card{number, id, winning, own, match_count: None});
    }
    cards
}

pub fn part1(lines: &[String]) -> String {

    let re_parsers = gen_re_parsers(); 
    let cards = parse_cards(lines, &re_parsers);
    let mut sum = 0;
    for mut card in cards {
        sum += card.points();
    }
    format!("{}", sum)
}

pub fn part2(lines: &[String]) -> String {
    let re_parsers = gen_re_parsers(); 
    let mut cards = parse_cards(lines, &re_parsers);
    let max_card_id = cards.len();
    let mut card_num_pile: Vec<usize> = cards.iter().map(|c| c.id).collect();
    let mut card_num_pile_index = 0;
    while card_num_pile_index < card_num_pile.len() {
        let card_id = card_num_pile[card_num_pile_index];
        let card = & mut cards[card_id];
        let mut w_c_c = card.winning_card_ids(max_card_id);
        card_num_pile.append(& mut w_c_c);
        card_num_pile_index += 1;
    }
    format!("{}", card_num_pile.len())
}



pub fn run() {
    let part1_file_path = "data/day04/part1.txt";
    let part2_file_path = "data/day04/part2.txt";
    println!("Result part 1 = {}", part1(&load_lines_from_file(part1_file_path).unwrap()));
    println!("Result part 2 = {}", part2(&load_lines_from_file(part2_file_path).unwrap()));
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let test_val = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let exp_result = "13";
        assert_eq!(exp_result, part1(&load_lines_from_str(test_val)));
    }


    #[test]
    fn test_part2() {
        let test_val = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let exp_result = "30";
        assert_eq!(exp_result, part2(&load_lines_from_str(test_val)));
    }
}
