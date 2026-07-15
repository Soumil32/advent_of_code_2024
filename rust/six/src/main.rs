use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Guard {
    position: Position,
    direction: Direction,
}

type PreparedInput = (Vec<Position>, HashSet<Position>, Guard, usize, usize);

fn main() {
    let input = read_to_string("input_example.txt").expect("Unable to read file");
    let mut input = prepare_input(&input);
    let part1 = one(&input);
    println!("Part 1: {}", part1.0);
    input.0 = part1.1;
    println!("Part 2: {}", two(&input));
}

fn prepare_input(input: &str) -> PreparedInput {
    let mut guard = Guard {
        position: Position { x: 0, y: 0 },
        direction: Direction::Right,
    };
    let mut visited = Vec::new();
    let mut obstacles = HashSet::new();

    let direction_map = HashMap::from([
        ('^', Direction::Up),
        ('>', Direction::Right),
        ('v', Direction::Down),
        ('<', Direction::Left),
    ]);
    let mut width = 0;
    let mut height = 0;
    for (row, line) in input.lines().enumerate() {
        height = row;
        for (column, char) in line.chars().enumerate() {
            width = column;
            if char == '#' {
                obstacles.insert(Position {
                    x: column as i32,
                    y: row as i32,
                });
                continue;
            }
            if ['^', '>', 'v', '<'].contains(&char) {
                guard.position = Position {
                    x: column as i32,
                    y: row as i32,
                };
                guard.direction = direction_map[&char].clone();
                visited.push(guard.position.clone());
                continue;
            }
        }
    }

    return (visited, obstacles, guard, width, height);
}

fn one(input: &PreparedInput) -> (u32, Vec<Position>) {
    let rotation_map = HashMap::from([
        (Direction::Up, Direction::Right),
        (Direction::Right, Direction::Down),
        (Direction::Down, Direction::Left),
        (Direction::Left, Direction::Up),
    ]);
    let (mut visited, obstacles, mut guard, width, height) = input.clone();

    loop {
        guard = calculate_next_guard_position(&guard, &obstacles, &rotation_map);
        if guard.position.x < 0
            || guard.position.x > width as i32
            || guard.position.y < 0
            || guard.position.y > height as i32
        {
            break;
        }
        visited.push(guard.position.clone());
    }

    //deduplicate visited
    visited.sort();
    visited.dedup();

    (visited.len() as u32, visited)
}

fn calculate_next_guard_position(
    guard: &Guard,
    obstacles: &HashSet<Position>,
    rotation_map: &HashMap<Direction, Direction>,
) -> Guard {
    let mut next_position;
    let mut current_guard = (*guard).clone();
    loop {
        next_position = current_guard.position.clone();
        match current_guard.direction {
            Direction::Up => next_position.y -= 1,
            Direction::Right => next_position.x += 1,
            Direction::Down => next_position.y += 1,
            Direction::Left => next_position.x -= 1,
        };
        if obstacles.contains(&next_position) {
            current_guard.direction = rotation_map[&current_guard.direction].clone();
        } else {
            current_guard.position = next_position;
            break;
        }
    }
    current_guard
}

fn two(visited: &PreparedInput) -> u32 {
    let rotation_map = HashMap::from([
        (Direction::Up, Direction::Right),
        (Direction::Right, Direction::Down),
        (Direction::Down, Direction::Left),
        (Direction::Left, Direction::Up),
    ]);

    let (visited, mut obstacles, mut guard, width, height) = visited.clone();
    let direction = guard.direction.clone();
    let mut total_posibilities = 0;

    for position in visited.iter() {
        let mut position_history: VecDeque<Position> = VecDeque::new();
        let mut new_visited: Vec<Position> = visited.clone();
        guard.position = position.clone();
        let next_state = calculate_next_guard_position(&guard, &obstacles, &rotation_map);

        obstacles.insert(next_state.position.clone());
        loop {
            let next_state = calculate_next_guard_position(&next_state, &obstacles, &rotation_map);
            guard = next_state;
            if guard.position.x < 0
                || guard.position.x > width as i32
                || guard.position.y < 0
                || guard.position.y > height as i32
            {
                break;
            }
            new_visited.push(guard.position.clone());
            position_history.push_back(guard.position.clone());
            if position_history.len() > 2 {
                position_history.pop_front();
            }
            if position_history.len() > 0 {
                if is_subsequence(&new_visited, &position_history.make_contiguous().to_vec()) {
                    total_posibilities += 1;
                    break;
                }
            }
        }
        obstacles.remove(&next_state.position);
    }

    total_posibilities
}

fn is_subsequence<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if haystack.is_empty() {
        return false;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    for i in 0..=(haystack.len() - needle.len()) {
        if &haystack[i..i + needle.len()] == needle {
            return true;
        }
    }
    false
}
