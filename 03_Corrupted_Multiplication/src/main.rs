use std::fs;

use regex::Regex;

fn main() {
    let input_string = fs::read_to_string("./input.txt").unwrap();

    let mut grand_total = 0;
    for line in input_string.split("\n") {
        let tuples = parse_tuples(line);
        let total: i32 = tuples.iter().map(|(a, b)| a * b).sum();
        grand_total += total
    }

    println!("Part 1 - Grand Total: {:?}", grand_total);
}

fn parse_tuples(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let tuples: Vec<(i32, i32)> = re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .collect();

    return tuples;
}
