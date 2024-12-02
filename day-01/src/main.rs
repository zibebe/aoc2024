use std::fs;

fn main() {
    let input = fs::read_to_string("day-01/input.txt").expect("input.txt file is missing");
    let mut left_ids = Vec::new();
    let mut right_ids = Vec::new();
    for (left, right) in input.split('\n').filter_map(|x| x.split_once("   ")) {
        let left: i32 = left.parse().unwrap();
        let right: i32 = right.parse().unwrap();
        left_ids.push(left);
        right_ids.push(right);
    }
    left_ids.sort();
    right_ids.sort();
    let mut total_distance = 0;
    for (left, right) in left_ids.iter().zip(right_ids.iter()) {
        total_distance += (left - right).abs();
    }
    println!("Answer: {}", total_distance);
}
