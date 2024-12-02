use std::fs;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

type Level = u32;

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
        let safe = check_report(line);
        if safe {
            safe_count += 1
        }
    }

    safe_count
}

fn check_report(report_string: &str) -> bool {
    let report: Vec<Level> = report_string
        .trim()
        .split(" ")
        .map(|x| x.parse::<Level>().unwrap())
        .collect();

    let length = report.len();

    for i in 0..length {
        let filtered_report = report
            .iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, x)| x);

        let safe = report_is_valid(filtered_report);
        if safe {
            return true;
        }
    }

    return false;
}

fn report_is_valid<'a, I>(levels: I) -> bool
where
    I: Iterator<Item = &'a Level>,
{
    let mut prev_level: Option<Level> = None;
    let mut direction: Option<Direction> = None;
    for &level in levels {
        match prev_level {
            None => {
                prev_level = Some(level);
            }
            Some(prev) => {
                let diff = level.abs_diff(prev);
                if diff > MAX_DIFFERENCE || diff < MIN_DIFFERENCE {
                    return false;
                }

                let new_direction = match level > prev {
                    true => Some(Direction::Decreasing),
                    false => Some(Direction::Increasing),
                };

                if direction.is_none() {
                    direction = new_direction;
                }

                if direction != new_direction {
                    return false;
                } else {
                    prev_level = Some(level)
                }
            }
        }
    }
    return true;
}
