use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    println!("Part one: {}", one(&input));
}

fn one(input: &str) -> usize {
    let rules: Vec<usize> = parse_rules(&input).unwrap();
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let updates: Vec<Vec<usize>> = input[1]
        .split("\n")
        .map(|s| s.trim())
        .map(|s| {
            s.split(",")
                .map(|num| num.parse::<usize>().unwrap())
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

fn parse_rules(input: &str) -> Result<Vec<usize>, &'static str> {
    // Collect all unique page numbers
    let mut pages_set = HashSet::new();
    let mut edges = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 2 {
            continue; // Invalid rule, skip
        }
        let x = parts[0].parse::<usize>().unwrap();
        let y = parts[1].parse::<usize>().unwrap();
        pages_set.insert(x);
        pages_set.insert(y);
        edges.push((x, y));
    }

    // Assign each page a unique index
    let mut pages: Vec<usize> = pages_set.into_iter().collect();
    pages.sort(); // Optional: to have pages in sorted order
    let page_to_index: HashMap<usize, usize> = pages.iter().enumerate().map(|(i, &page)| (page, i)).collect();

    // Build adjacency list and in-degree array
    let num_nodes = pages.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); num_nodes];
    let mut in_degree: Vec<usize> = vec![0; num_nodes];
    let mut edge_set = HashSet::new();

    for (x, y) in edges {
        let i = *page_to_index.get(&x).unwrap();
        let j = *page_to_index.get(&y).unwrap();
        if !edge_set.contains(&(i, j)) {
            adj[i].push(j);
            in_degree[j] += 1;
            edge_set.insert((i, j));
        }
    }

    // Perform topological sort
    let mut queue = VecDeque::new();
    for i in 0..num_nodes {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut result = Vec::new();
    while let Some(u) = queue.pop_front() {
        result.push(u);
        for &v in &adj[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    if result.len() != num_nodes {
        Err("Cycle detected in the rules")
    } else {
        // Map indices back to page numbers
        let ordered_pages: Vec<usize> = result.into_iter().map(|i| pages[i]).collect();
        Ok(ordered_pages)
    }
}
