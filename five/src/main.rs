use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    println!("Part one: {}", one(&input));
}

fn one(input: &str) -> i32 {
    let rules: Vec<i32> = parse_rules(&input);
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let updates: Vec<Vec<i32>> = input[1]
        .split("\n")
        .map(|s| s.trim())
        .map(|s| {
            s.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut total = 0;

    for update in updates.iter() {
        let mut sorted_update = update.clone();
        sorted_update.sort_by_key(|x| match rules.iter().position(|y| *y == *x) {
            Some(i) => {i},
            None => {panic!("Could not find {} in rules", x)},
        });
        if sorted_update == *update {
            // find the middle number in the list (the length of the list is odd)
            let middle = sorted_update.len() / 2;
            total += sorted_update[middle];
        }
    }
    total
}

fn parse_rules(input: &str) -> Vec<i32> {
    // Initialize a HashSet to store unique page numbers and a vector to store edges between pages
    let mut pages_set = HashSet::new();
    let mut edges = Vec::new();

    // Parse input string line by line to extract page relationships
    for line in input.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 2 {
            continue; // Skip invalid rules that don't have exactly two parts
        }
        // Parse the two numbers representing connected pages
        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[1].parse::<i32>().unwrap();
        // Add both pages to the set of unique pages
        pages_set.insert(x);
        pages_set.insert(y);
        // Store the edge connecting these pages
        edges.push((x, y));
    }

    // Convert the set of unique pages to a sorted vector
    let mut pages: Vec<i32> = pages_set.into_iter().collect();
    pages.sort(); // Sort pages for consistent ordering
    // Create a mapping from page numbers to their indices in the sorted array
    let page_to_index: HashMap<i32, i32> = pages
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i as i32))
        .collect();

    // Initialize data structures for topological sort
    let num_nodes = pages.len();
    let mut adj: Vec<Vec<i32>> = vec![Vec::new(); num_nodes];     // Adjacency list representation
    let mut in_degree: Vec<i32> = vec![0; num_nodes];             // Track incoming edges for each node
    let mut edge_set = HashSet::new();                            // Track unique edges

    // Build the graph representation
    for (x, y) in edges {
        let i = page_to_index[&x];
        let j = page_to_index[&y];
        if !edge_set.contains(&(i, j)) {
            adj[i as usize].push(j);           // Add directed edge
            in_degree[j as usize] += 1;        // Increment in-degree of destination
            edge_set.insert((i, j));           // Mark edge as processed
        }
    }

    // Initialize queue with nodes that have no incoming edges
    let mut queue = VecDeque::new();
    for i in 0..num_nodes {
        if in_degree[i] == 0 {
            queue.push_back(i as i32);
        }
    }

    // Perform topological sort using Kahn's algorithm
    let mut result = Vec::new();
    while let Some(u) = queue.pop_front() {
        result.push(u);
        // Process all neighbors of the current node
        for &v in &adj[u as usize] {
            in_degree[v as usize] -= 1;        // Remove edge by decreasing in-degree
            if in_degree[v as usize] == 0 {    // If node has no more incoming edges
                queue.push_back(v);            // Add it to the queue
            }
        }
    }
    // Convert indices back to original page numbers and return
    let ordered_pages: Vec<i32> = result.into_iter().map(|i| pages[i as usize]).collect();

    ordered_pages
}
