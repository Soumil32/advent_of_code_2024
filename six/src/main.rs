use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Spot {
    Empty,
    Visited,
    Obstacle,
    Guard,
}

#[derive(Debug, PartialEq, Eq)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
}

fn main() {
    let input = read_to_string("input_example.txt").expect("Unable to read file");
    println!("Part 1: {}", one(&input));
}

fn one(input: &str) -> u32 {
    let mut guard = Guard {
        position: (0, 0),
        direction: Direction::Right,
    };
    let mut grid: Vec<Vec<Spot>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let i = i as i32;
            let j = j as i32;
            let spot = match c {
                '.' => Spot::Empty,
                '#' => Spot::Obstacle,
                '^' => {
                    guard.position = (i, j);
                    guard.direction = Direction::Up;
                    Spot::Guard
                }
                'v' => {
                    guard.position = (i, j);
                    guard.direction = Direction::Down;
                    Spot::Guard
                }
                '<' => {
                    guard.position = (i, j);
                    guard.direction = Direction::Left;
                    Spot::Guard
                }
                '>' => {
                    guard.position = (i, j);
                    guard.direction = Direction::Right;
                    Spot::Guard
                }
                _ => panic!(),
            };

            row.push(spot);
        }
        grid.push(row);
    }

    assert!(grid.len() > 0);
    assert!(grid[0].len() > 0);
    let mut visited = 0;

    loop {
        let success = calculate_next_position(&mut guard, &grid);
        grid[guard.position.0 as usize][guard.position.1 as usize] = Spot::Visited;
        if success {
            break;
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let spot = &grid[i][j];
            if *spot == Spot::Visited {
                visited += 1;
            }
        }
    }

    visited
}

fn calculate_next_position(guard: &mut Guard, grid: &Vec<Vec<Spot>>) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    loop {
        let current_position = guard.position;
        let next_position = match guard.direction {
            Direction::Up => (current_position.0.saturating_sub(1), current_position.1),
            Direction::Down => (current_position.0 + 1, current_position.1),
            Direction::Left => (current_position.0, current_position.1.saturating_sub(1)),
            Direction::Right => (current_position.0, current_position.1 + 1),
        };
        if next_position.1 < 0 || next_position.1 >= width || next_position.0 < 0 || next_position.0 >= height {
            return true;
        }  

        if grid[next_position.0 as usize][next_position.1 as usize] == Spot::Obstacle {
            guard.direction = rotate(&guard.direction);
        } else {
            guard.position = next_position;
            if guard.position.0 < 0 || guard.position.0 >= height || guard.position.1 < 0 || guard.position.1 >= width {
                return true;
            }
            return false;
        }

    }
    
}

fn rotate(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
