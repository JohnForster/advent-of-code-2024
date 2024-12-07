use std::{fs, num::TryFromIntError, ops::Deref};

struct Wordsearch {
    word: String,
    height: usize,
    width: usize,
    chars: Vec<char>,
}

impl Deref for Wordsearch {
    type Target = Vec<char>;

    fn deref(&self) -> &Vec<char> {
        &self.chars
    }
}

impl Wordsearch {
    fn new(input_string: &str, word: &str) -> Self {
        let lines: Vec<&str> = input_string.trim().split("\n").collect();
        let height = lines.len();
        let chars: Vec<char> = lines.iter().flat_map(|line| line.trim().chars()).collect();
        let width = chars.len() / height;

        Wordsearch {
            word: word.to_string(),
            height,
            width,
            chars,
        }
    }

    pub fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x.is_negative()
            || y.is_negative()
            || x as usize >= self.width
            || y as usize >= self.height
        {
            return None;
        }

        let index = self.width * y as usize + x as usize;
        return Some(index);
    }

    pub fn element_at(&self, x: i32, y: i32) -> Option<char> {
        let index = self.get_index(x, y)?;
        return Some(self.chars[index]);
    }

    pub fn check_char(&self, index: usize) -> usize {
        let x0 = (index % self.width) as i32;
        let y0 = (index / self.width) as i32;

        let mut solutions: usize = 0;
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if dx == 0 as i32 && dy == 0 as i32 {
                    continue;
                }

                let found = self.word.chars().enumerate().all(|(i, letter)| {
                    let x = x0 + dx * (i as i32);
                    let y = y0 + dy * (i as i32);

                    if let Some(char) = self.element_at(x, y) {
                        return char == letter;
                    } else {
                        return false;
                    };
                });

                if found {
                    // println!("({:?}, {:?}), [{:?}, {:?}]", x0, y0, dx, dy);
                    solutions += 1;
                }
            }
        }
        return solutions;
    }

    fn count_crosses(&self) -> usize {
        self.iter()
            .enumerate()
            .filter(|(i, char)| self.check_char_for_cross(*i))
            .count()
    }

    fn check_char_for_cross(&self, index: usize) -> bool {
        let x0 = (index % self.width) as i32;
        let y0 = (index / self.width) as i32;
        if self[index] == 'A' {
            let a1 = self.element_at(x0 - 1, y0 - 1) == Some('M')
                && self.element_at(x0 + 1, y0 + 1) == Some('S');
            let a2 = self.element_at(x0 - 1, y0 - 1) == Some('S')
                && self.element_at(x0 + 1, y0 + 1) == Some('M');
            let b1 = self.element_at(x0 - 1, y0 + 1) == Some('S')
                && self.element_at(x0 + 1, y0 - 1) == Some('M');
            let b2 = self.element_at(x0 - 1, y0 + 1) == Some('M')
                && self.element_at(x0 + 1, y0 - 1) == Some('S');

            return (a1 || a2) && (b1 || b2);
        } else {
            return false;
        }
    }

    fn count_solutions(&self) -> usize {
        self.iter()
            .enumerate()
            .map(|(i, char)| if *char == 'X' { self.check_char(i) } else { 0 })
            .sum()
    }
}

fn main() {
    let input_string = fs::read_to_string("./input.txt").unwrap();

    let wordsearch = Wordsearch::new(input_string.trim(), "XMAS");
    let total = wordsearch.count_solutions();
    println!("solutions: {:?}", total);
    let total = wordsearch.count_crosses();
    println!("crosses: {:?}", total);
}

#[test]
fn test_forwards() {
    let input = "
        XMAS.
        MM...
        A.A..
        S..S.
        ....."
        .trim();
    let worsearch = Wordsearch::new(input, "XMAS");
    let result = worsearch.count_solutions();
    assert_eq!(result, 3);
}

#[test]
fn test_backwards() {
    let input = "
        SAMX.....
        AA.......
        M.M......
        X..X.....
        ........."
        .trim();
    let worsearch = Wordsearch::new(input, "XMAS");
    let result = worsearch.count_solutions();
    assert_eq!(result, 3);
}

#[test]
fn test_both() {
    let input = "
        XMAS.SAMX.
        MM...AA...
        A.A..M.M..
        S..S.X..X."
        .trim();
    let worsearch = Wordsearch::new(input, "XMAS");
    let result = worsearch.count_solutions();
    assert_eq!(result, 6);
}

#[test]
fn test_star() {
    let input = "
    .S..S..S.
    ..A.A.A..
    ...MMM...
    .SAMXMAS.
    ...MMM...
    ..A.A.A..
    .S..S..S.
    "
    .trim();
    let worsearch = Wordsearch::new(input, "XMAS");
    let result = worsearch.count_solutions();
    assert_eq!(result, 8);
}

#[test]
fn test_given_test_input() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let worsearch = Wordsearch::new(input, "XMAS");
    let result = worsearch.count_solutions();
    assert_eq!(result, 18);
}
