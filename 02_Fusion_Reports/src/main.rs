use colored::{ColoredString, Colorize};
use std::fs;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

type Level = i32;

const MAX_DIFFERENCE: Level = 3;
const MIN_DIFFERENCE: Level = 1;
fn main() {
    let input_string = fs::read_to_string("./input.txt").unwrap();

    let safe_count = check_input(&input_string);

    println!("Part 2 - Safe Count: {:?}", safe_count);
}

fn check_input(input_string: &str) -> i32 {
    let mut safe_count = 0;

    for line in input_string.trim().split("\n") {
        let mut safe = check_report(line);
        if !safe {
            let levels: Vec<&str> = line.trim().split(" ").collect();
            let length = levels.len();
            'level: for i in 0..length {
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                let new_safe = check_report(&new_levels.join(" "));

                if new_safe {
                    safe = true;
                    break 'level;
                }
            }
        }

        if safe {
            safe_count += 1;
        }
    }

    safe_count
}

fn check_report(line: &str) -> bool {
    let mut prev_level: Option<Level> = None;
    let mut direction: Option<Direction> = None;

    for level in line.split(" ").map(|x| x.parse::<Level>().unwrap()) {
        match prev_level {
            None => {
                prev_level = Some(level);
            }
            Some(prev) => {
                let difference = level - prev;
                let new_direction = match difference.is_positive() {
                    true => Direction::Decreasing,
                    false => Direction::Increasing,
                };

                if direction.is_none() {
                    direction = Some(new_direction);
                }

                if difference.abs() > MAX_DIFFERENCE
                    || difference.abs() < MIN_DIFFERENCE
                    || direction.unwrap() != new_direction
                {
                    return false;
                } else {
                    prev_level = Some(level)
                }
            }
        }
    }
    return true;
}
