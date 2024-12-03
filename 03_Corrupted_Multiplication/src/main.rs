use std::fs;

use regex::Regex;

fn main() {
    let input_string = fs::read_to_string("./input.txt").unwrap();
    let part_1 = part_1(&input_string);
    println!("Part 1 - Grand Total: {:?}", part_1);
    let part_2 = part_2(&input_string);
    println!("Part 2 - Grand Total: {:?}", part_2);
}

fn part_1(input: &str) -> i32 {
    let tuples = parse_tuples(input);
    let total = multiply_and_sum_tuples(tuples);
    total
}

fn multiply_and_sum_tuples(tuples: Vec<(i32, i32)>) -> i32 {
    tuples.iter().map(|(a, b)| a * b).sum()
}

fn part_2(input: &str) -> i32 {
    let filtered_instructions = filter_instructions(input);
    let tuples = parse_tuples(&filtered_instructions);
    let total = multiply_and_sum_tuples(tuples);
    total
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

fn filter_instructions(line: &str) -> String {
    line.split("do()")
        .map(|str| truncate_at_first_dont(str))
        .collect::<Vec<&str>>()
        .join("")
}

fn truncate_at_first_dont(str: &str) -> &str {
    str.split("don't").next().unwrap()
}

#[test]
fn test_a_do_string() {
    let input = "mul(4,5)do()mul(5,6)do()mul(6,7)";
    let result = part_2(input);
    assert_eq!(result, 92);
}

#[test]
fn test_a_dont_string() {
    let input = "mul(4,5)mul(2,5)don't()mul(3,6)";
    let result = part_2(input);
    assert_eq!(result, 30);
}

#[test]
fn test_a_dont_do_string() {
    let input = "mul(4,5)mul(2,5)don't()mul(3,6)do()mul(3,4)";
    let result = part_2(input);
    assert_eq!(result, 42);
}

#[test]
fn test_a_do_dont_string() {
    let input = "mul(4,5)do()mul(2,5)don't()mul(3,6)-mul(3,4)";
    let result = part_2(input);
    assert_eq!(result, 30);
}

#[test]
fn test_a_do_dont_do_string() {
    let input = "mul(4,5)do()mul(2,5)don't()mul(3,6)-mul(3,4)do()mul(3,2)";
    let result = part_2(input);
    assert_eq!(result, 36);
}

#[test]
fn test_a_multi_line_string() {
    let input = "mul(4,5)do()mul(2,5)don't()\nmul(3,6)-mul(3,4)do()mul(3,2)";
    let result = part_2(input);
    assert_eq!(result, 36);
}
