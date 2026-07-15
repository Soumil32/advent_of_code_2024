use std::{char, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", one(&input));
    println!("{}", two(&input));
}

fn one(input: &str) -> u32 {
    let rows = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut total_christmases = 0;
    for (i, row) in rows.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == 'X' {
                // search to the right for 'XMAS'
                if j + 3 < row.len() {
                    let next_three = row[j+1..j+4].to_vec();
                    assert_eq!(next_three.len(), 3);
                    if next_three == vec!['M', 'A', 'S' ] {
                        total_christmases += 1;
                    }
                }
                // search to the left for 'XMAS'
                if j as isize - 3 >= 0 {
                    let last_three = row[j-3..j].to_vec();
                    assert_eq!(last_three.len(), 3);
                    if last_three == vec!['S', 'A', 'M' ] {
                        total_christmases += 1;
                    }
                }
                // search to the down for 'XMAS'
                if i + 3 < rows.len() && rows[i+1..i+4].iter().map(|row| row[j]).collect::<Vec<char>>() == vec!['M', 'A', 'S' ] {
                    let down_three = rows[i+1..i+4].iter().map(|row| row[j]).collect::<Vec<char>>();
                    assert_eq!(down_three.len(), 3);
                    if down_three == vec!['M', 'A', 'S' ] {
                        total_christmases += 1;
                    }
                }
                // search to the up for 'XMAS'
                if i as isize - 3 >= 0 && rows[i-3..i].iter().map(|row| row[j]).collect::<Vec<char>>() == vec!['S', 'A', 'M' ] {
                    let up_three = rows[i-3..i].iter().map(|row| row[j]).collect::<Vec<char>>();
                    assert_eq!(up_three.len(), 3);
                    if up_three == vec!['S', 'A', 'M' ] {
                        total_christmases += 1;
                    }
                }
                // search to the down-right for 'XMAS'
                if i + 3 < rows.len() && j + 3 < row.len() {
                    let down_right_three = rows[i+1..i+4].iter().enumerate().map(|(index, row)| row[j+index+1]).collect::<Vec<char>>();
                    assert_eq!(down_right_three.len(), 3);
                    if down_right_three == vec!['M', 'A', 'S' ] {
                        total_christmases += 1;
                    }
                }
                // search to the up-left for 'XMAS'
                if i as isize - 3 >= 0 && j as isize - 3 >= 0 {
                    let up_left_three = rows[i-3..i].iter().enumerate().map(|(index, row)| row[j-(3-index)]).collect::<Vec<char>>();
                    assert_eq!(up_left_three.len(), 3);
                    if up_left_three == vec!['S', 'A', 'M' ] {
                        total_christmases += 1;
                    }
                }
                // search to the up-right for 'XMAS'
                if i as isize - 3 >= 0 && j + 3 < row.len() {
                    let up_right_three = rows[i-3..i].iter().enumerate().map(|(index, row)| row[j+(3-index)]).collect::<Vec<char>>();
                    assert_eq!(up_right_three.len(), 3);
                    if up_right_three == vec!['S', 'A', 'M' ] {
                        total_christmases += 1;
                    }
                }
                // search to the down-left for 'XMAS'
                if i + 3 < rows.len() && j as isize - 3 >= 0 {
                    let down_left_three = rows[i+1..i+4].iter().enumerate().map(|(index, row)| {row[j-index-1]}).collect::<Vec<char>>();
                    assert_eq!(down_left_three.len(), 3);
                    if down_left_three == vec!['M', 'A', 'S' ] {
                        total_christmases += 1;
                    }
                }
        
            }
        }
    }

    total_christmases
}

fn two(input: &str) -> u32 {
    let rows = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut total_christmases = 0;

    for (i, row) in rows.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == 'A' && i >=1 && i < rows.len() - 1 && j >= 1 && j < row.len() - 1 {
                let top_left = rows[i-1][j-1];
                let top_right = rows[i-1][j+1];
                let bottom_left = rows[i+1][j-1];
                let bottom_right = rows[i+1][j+1];
                if top_left == 'M' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'S' {
                    total_christmases += 1;
                }
                if top_left == 'S' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'M' {
                    total_christmases += 1;
                }
                if top_left == 'M' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'S' {
                    total_christmases += 1;
                }
                if top_left == 'S' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'M' {
                    total_christmases += 1;
                }
            }
        }
    }

    total_christmases
}
