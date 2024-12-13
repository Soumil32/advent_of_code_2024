use std::{char, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", one(&input));
}

fn one(input: &str) -> u32 {
    let rows = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut total_christmases = 0;
    for (i, row) in rows.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == 'X' {
                // search to the right for 'XMAS'
                if j + 4 < row.len() && row[j+1..j+4].to_vec() == vec!['M', 'A', 'S' ] {
                    total_christmases += 1;
                }
                // search to the left for 'XMAS'
                if j as isize - 4 >= 0 && row[j-4..j-1].to_vec() == vec!['S', 'A', 'M' ] {
                    total_christmases += 1;
                }
                // search to the down for 'XMAS'
                if i + 4 < rows.len() && rows[i+1..i+4].to_vec() == vec![vec!['M', 'A', 'S' ]] {
                    total_christmases += 1;
                } else {
                    println!("{:?}", rows[i..i+4].to_vec());
                }
            }
        }
    }

    total_christmases
}
