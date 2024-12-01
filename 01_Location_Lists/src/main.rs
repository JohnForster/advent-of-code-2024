use std::{collections::HashMap, fs};

fn main() {
    let mut left_list: Vec<usize> = vec![];
    let mut right_list: Vec<usize> = vec![];
    let contents = fs::read_to_string("./input.txt").unwrap();
    for line in contents.split("\n") {
        let ids: Vec<&str> = line.split("   ").map(|id| id.trim()).collect();
        let id1 = ids[0].parse::<usize>().unwrap();
        let id2 = ids[1].parse::<usize>().unwrap();
        left_list.push(id1);
        right_list.push(id2);
    }

    left_list.sort();
    right_list.sort();

    let mut total_distance = 0;
    for (i, id1) in left_list.iter().enumerate() {
        let id2 = right_list[i];
        let d = id1.abs_diff(id2);
        total_distance += d
    }

    println!("PART 1: total_distance: {:?}", total_distance);

    let mut right_occurances: HashMap<usize, usize> = HashMap::new();
    for id in right_list.iter() {
        match right_occurances.get_mut(id) {
            Some(n) => *n += 1,
            None => {
                right_occurances.insert(*id, 1);
            }
        }
    }

    let mut similarity = 0;
    for id in left_list.iter() {
        if let Some(n) = right_occurances.get(id) {
            similarity += id * n
        }
    }

    println!("PART 2: similarity: {:?}", similarity);
}
