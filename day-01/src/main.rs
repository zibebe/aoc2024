use std::fs;

fn main() {
    let input = fs::read_to_string("day-01/input.txt").expect("input.txt file is missing");

    // Create a Vector for the left and right elements and parse them to i32
    let (mut left_ids, mut right_ids): (Vec<i32>, Vec<i32>) = input
        .split('\n')
        .filter_map(|x| x.split_once("   "))
        .map(|(left, right)| {
            let left: i32 = left.parse().unwrap();
            let right: i32 = right.parse().unwrap();
            (left, right)
        })
        .unzip();

    // Sort them ascending
    left_ids.sort_unstable();
    right_ids.sort_unstable();

    // Calculate the total distance, which is the difference between each value on the left and right
    // summed up
    let total_distance: i32 = left_ids
        .iter()
        .zip(right_ids.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("Answer of Part 1: {}", total_distance);
}
