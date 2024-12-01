use std::{collections::HashMap, fs};

fn main() {
    let (left_list, right_list) = construct_sorted_lists("./input.txt");
    let total_distance = calculate_distance(&left_list, &right_list);
    println!("PART 1 - total_distance: {:?}", total_distance);

    let right_occurances = count_occurances(&right_list);
    let similarity = calculate_similarity(&left_list, &right_occurances);
    println!("PART 2 - similarity: {:?}", similarity);
}

fn construct_sorted_lists(path: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left_list: Vec<usize> = vec![];
    let mut right_list: Vec<usize> = vec![];
    let contents = fs::read_to_string(path).unwrap();
    for line in contents.trim().split("\n") {
        let ids: Vec<&str> = line.split("   ").map(|id| id.trim()).collect();
        let left_id = ids[0].parse::<usize>().unwrap();
        let right_id = ids[1].parse::<usize>().unwrap();
        left_list.push(left_id);
        right_list.push(right_id);
    }

    left_list.sort();
    right_list.sort();
    (left_list, right_list)
}

fn calculate_distance(left_list: &Vec<usize>, right_list: &Vec<usize>) -> usize {
    let mut total_distance = 0;
    for (i, left_id) in left_list.iter().enumerate() {
        let right_id = right_list[i];
        let diff = left_id.abs_diff(right_id);
        total_distance += diff
    }
    total_distance
}

fn count_occurances(id_list: &Vec<usize>) -> HashMap<usize, usize> {
    let mut occurances: HashMap<usize, usize> = HashMap::new();
    for id in id_list.iter() {
        match occurances.get_mut(id) {
            Some(n) => *n += 1,
            None => {
                occurances.insert(*id, 1);
            }
        }
    }
    occurances
}

fn calculate_similarity(left_list: &Vec<usize>, right_occurances: &HashMap<usize, usize>) -> usize {
    let mut similarity = 0;
    for id in left_list.iter() {
        if let Some(n) = right_occurances.get(id) {
            similarity += id * n
        }
    }
    similarity
}
