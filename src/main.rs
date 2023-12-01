use std::vec;

#[derive(Debug, PartialEq)]
pub struct Number {
    idx: usize,
    val: usize,
    nan: bool,
}

impl Number {
    fn new(idx: usize, val: usize, nan: bool) -> Self {
        Self { idx, val, nan }
    }
}

const RADIX: u32 = 10;
const WORD_NUMS: [&str; 10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

fn main() {
    let input = include_str!("inputs/day1/input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let numbers: Vec<Vec<Number>> = parse_numbers(&lines);

    let part1 = part1(&numbers);
    println!("Part 1: {}", part1);

    let part2 = part2(&numbers);
    println!("Part 2: {}", part2);
}

fn parse_numbers(records: &Vec<&str>) -> Vec<Vec<Number>> {
    let mut output: Vec<Vec<Number>> = vec![];
    for record in records {
        let mut digits: Vec<Number> = parse_digits(record);
        let mut words: Vec<Number> = parse_words(record);

        digits.append(&mut words);
        let mut numbers = digits;
        numbers.sort_by(|a, b| a.idx.cmp(&b.idx));
        output.push(numbers);
    }
    output
}

fn parse_words(record: &str) -> Vec<Number> {
    let mut output: Vec<Number> = vec![];
    for word in WORD_NUMS {
        for (idx, _word) in record.match_indices(word) {
            output.push(Number::new(idx, convert(word), true))
        }
    }
    output
}

fn parse_digits(record: &str) -> Vec<Number> {
    let mut output: Vec<Number> = vec![];
    for (idx, _char) in format!("{record}*").chars().enumerate() {
        if _char.is_digit(RADIX) {
            let val: usize = _char.to_digit(RADIX).unwrap() as usize;
            output.push(Number::new(idx, val, false));
        }
    }
    output
}

fn convert(word: &str) -> usize {
    return match word {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        _ => panic!("Number not in 0-10"),
    };
}

fn part1(numbers: &Vec<Vec<Number>>) -> usize {
    let mut solution: usize = 0;
    for record in numbers {
        // Part 1 only cares about digits, not spelled out words
        let first = match record.iter().find(|n| !n.nan) {
            Some(n) => n.val,
            None => 0,
        };
        let last = match record.iter().rev().find(|n| !n.nan) {
            Some(n) => n.val,
            None => 0,
        };
        solution += first * 10 + last;
    }
    solution
}

fn part2(numbers: &Vec<Vec<Number>>) -> usize {
    let mut solution: usize = 0;
    for record in numbers {
        // Part 2 does not care at all
        let first = match record.first() {
            Some(n) => n.val,
            None => 0,
        };
        let last = match record.last() {
            Some(n) => n.val,
            None => 0,
        };
        solution += first * 10 + last;
    }
    solution
}
