use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("part one: {}", one(&input));
    println!("part two: {}", two(&input));
}

fn one(input : &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let matches = re.find_iter(input);
    let mut sum = 0;
    for mul in matches {
        sum += parse_multiply(&mul.as_str());
    }
    sum
}

fn two(input : &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
    let matches = re.find_iter(input);
    let mut mul_enabled = true;
    let mut sum = 0;
    for opperator in matches {
        if opperator.as_str() == "don't()" {
            mul_enabled = false;
        } else if opperator.as_str() == "do()" {
            mul_enabled = true;
        } else if mul_enabled {
            sum += parse_multiply(&opperator.as_str());
        }
    }

    sum
}

fn parse_multiply(input : &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let caps = re.captures(input).unwrap();
    let num1 = caps[1].parse::<i32>().unwrap();
    let num2 = caps[2].parse::<i32>().unwrap();
    num1 * num2
}
