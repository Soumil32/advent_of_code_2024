use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    println!("Part one: {}", one(&input));
    println!("Part two: {}", two(&input));
}

fn one(input: &str) -> usize {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let rules: Vec<Vec<usize>> = input[0]
        .split("\n")
        .map(|s| s.trim().split("|").map(|n| n.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();


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
        let mut useful_rules = vec![];
        for rule in rules.iter() {
            if update.contains(&rule[0]) && update.contains(&rule[1]) {
                useful_rules.push(rule);
            }
        }
        let ordered_rules = parse_rules(&useful_rules);
        let mut sorted_update = update.clone();
        sorted_update.sort_by_key(|x| ordered_rules.iter().position(|&y| y == *x).unwrap());
        if sorted_update == *update {
            // find the middle number in the list (the length of the list is odd)
            let middle = sorted_update.len() / 2;
            total += sorted_update[middle];
        }
    }
    total
}

fn two(input: &str) -> usize {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let rules: Vec<Vec<usize>> = input[0]
        .split("\n")
        .map(|s| s.trim().split("|").map(|n| n.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();


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
        let mut useful_rules = vec![];
        for rule in rules.iter() {
            if update.contains(&rule[0]) && update.contains(&rule[1]) {
                useful_rules.push(rule);
            }
        }
        let ordered_rules = parse_rules(&useful_rules);
        let mut sorted_update = update.clone();
        sorted_update.sort_by_key(|x| ordered_rules.iter().position(|&y| y == *x).unwrap());
        if sorted_update != *update {
            // find the middle number in the list (the length of the list is odd)
            let middle = sorted_update.len() / 2;
            total += sorted_update[middle];
        }
    }
    total
}

fn parse_rules(input: &Vec<&Vec<usize>>) -> Vec<usize> {
    // Collect all unique page numbers
    let mut pages_set = HashSet::new();
    let mut edges = Vec::new();

    for line in input.iter() {
        if line.len() != 2 {
            continue; // Invalid rule, skip
        }
        let x = line[0];
        let y = line[1];
        pages_set.insert(x);
        pages_set.insert(y);
        edges.push((x, y));
    }

    // Assign each page a unique index
    let mut pages: Vec<usize> = pages_set.into_iter().collect();
    pages.sort(); // Optional: to have pages in sorted order
    let page_to_index: HashMap<usize, usize> = pages
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();
    let index_to_page: HashMap<usize, usize> = pages
        .iter()
        .enumerate()
        .map(|(i, &page)| (i, page))
        .collect();

    // Build adjacency list for the original graph
    let num_nodes = pages.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); num_nodes];
    for (x, y) in edges {
        let i = *page_to_index.get(&x).unwrap();
        let j = *page_to_index.get(&y).unwrap();
        adj[i].push(j);
    }

    // Function to perform DFS and return visited nodes
    fn dfs(node: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        if visited[node] {
            return;
        }
        visited[node] = true;
        for &neighbor in &adj[node] {
            dfs(neighbor, adj, visited, order);
        }
        order.push(node);
    }

    // Step 1: Perform DFS and push nodes onto the order stack
    let mut visited = vec![false; num_nodes];
    let mut order = Vec::new();
    for i in 0..num_nodes {
        if !visited[i] {
            dfs(i, &adj, &mut visited, &mut order);
        }
    }

    // Step 2: Reverse the graph
    let mut adj_rev: Vec<Vec<usize>> = vec![Vec::new(); num_nodes];
    for i in 0..num_nodes {
        for &neighbor in &adj[i] {
            adj_rev[neighbor].push(i);
        }
    }

    // Step 3: Perform DFS on the reversed graph in the order defined by 'order'
    let mut visited_rev = vec![false; num_nodes];
    let mut sccs = Vec::new();
    for &node in order.iter().rev() {
        if !visited_rev[node] {
            let mut scc = Vec::new();
            dfs_rev(node, &adj_rev, &mut visited_rev, &mut scc);
            sccs.push(scc);
        }
    }

    // Helper function for DFS on reversed graph
    fn dfs_rev(
        node: usize,
        adj_rev: &Vec<Vec<usize>>,
        visited_rev: &mut Vec<bool>,
        scc: &mut Vec<usize>,
    ) {
        if visited_rev[node] {
            return;
        }
        visited_rev[node] = true;
        scc.push(node);
        for &neighbor in &adj_rev[node] {
            dfs_rev(neighbor, adj_rev, visited_rev, scc);
        }
    }

    // Step 4: Build a graph of SCCs
    let mut scc_graph: Vec<Vec<usize>> = vec![Vec::new(); sccs.len()];
    let mut scc_id: HashMap<usize, usize> = HashMap::new();
    for (scc_id_idx, scc) in sccs.iter().enumerate() {
        for &node in scc {
            scc_id.insert(node, scc_id_idx);
        }
    }
    for (scc_id_idx, scc) in sccs.iter().enumerate() {
        for &node in scc {
            for &neighbor in &adj[node] {
                if scc_id[&neighbor] != scc_id_idx {
                    scc_graph[scc_id_idx].push(scc_id[&neighbor]);
                }
            }
        }
    }

    // Step 5: Perform topological sort on SCCs
    let mut scc_in_degree: Vec<usize> = vec![0; sccs.len()];
    for u in 0..sccs.len() {
        for &v in &scc_graph[u] {
            scc_in_degree[v] += 1;
        }
    }
    let mut queue = VecDeque::new();
    for u in 0..sccs.len() {
        if scc_in_degree[u] == 0 {
            queue.push_back(u);
        }
    }
    let mut scc_order = Vec::new();
    while let Some(u) = queue.pop_front() {
        scc_order.push(u);
        for &v in &scc_graph[u] {
            scc_in_degree[v] -= 1;
            if scc_in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    // Step 6: Build the final page order
    let mut final_order = Vec::new();
    for &scc_idx in &scc_order {
        for &node in &sccs[scc_idx] {
            final_order.push(node);
        }
    }
    // Append remaining pages (should be from cycles)
    for scc in sccs.iter().rev() {
        for &node in scc {
            if !final_order.contains(&node) {
                final_order.push(node);
            }
        }
    }

    // Map indices back to page numbers
    let ordered_pages: Vec<usize> = final_order.into_iter().map(|i| index_to_page[&i]).collect();

    ordered_pages
}
