use std::{fs, vec};

fn main() {
    let input = read_input_from_file("input.txt");
    let one = part_one(&input);
    println!("Part one: {}", one);
    let two = part_two(&input);
    println!("Part two: {}", two);
}

fn read_input_from_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read input file")
}

fn parse_list(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list = vec![];
    let mut right_list = vec![];
    input.lines().for_each(|line| {
        let (left, right) = line.split_once(' ').unwrap();
        let left: u32 = left.trim().parse().unwrap();
        let right: u32 = right.trim().parse().unwrap();
        left_list.push(left);
        right_list.push(right);
    });
    return (left_list, right_list)
}

fn part_one(input: &str) -> u32 {
    let (mut left_list, mut right_list) = parse_list(input);
    left_list.sort();
    right_list.sort();
    assert_eq!(left_list.len(), right_list.len());

    let mut total_distance = 0;
    for i in 0..left_list.len() {
        let left_item: u32 = left_list[i];
        let right_item: u32 = right_list[i];
        let distance;
        if left_item > right_item {
            distance = left_item - right_item;
        } else {
            distance = right_item - left_item;
        }
        total_distance += distance;
    }
    total_distance
}

fn part_two(input: &str) -> u32 {
    let (left_list, right_list) = parse_list(input);
    assert_eq!(left_list.len(), right_list.len());

    let mut total_similarity = 0;
    for location_id in left_list.iter() {
        // the number of times the item appears in the right list
        let count = right_list.iter().filter(|&x| x == location_id).count(); 
        total_similarity += location_id * count as u32;
    }
    total_similarity
}