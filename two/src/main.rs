use std::fs;
use std::vec::Vec;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let reports: Vec<Vec<u32>> = input.lines().map(|line| line.split(" ").map(|s| s.parse().unwrap()).collect()).collect();
    let mut safe_reports = 0;
    let mut unsafe_reports = 0;
    for report in reports.iter() {
        let mut is_safe = true;
        let mut is_accending = 0; // 0 - not accending, 1 - accending, 2 - decending
        for i in 1..report.len() {
            let current = report[i];
            let previous = report[i - 1];
            if current > previous {
                if is_accending == 0 {
                    is_accending = 1;
                } else if is_accending == 2 {
                    is_safe = false;
                    
                }

                let difference_between = current - previous;
                if difference_between < 1 || difference_between > 3 {
                    is_safe = false;
                    
                } 
            } else if current < previous {
                if is_accending == 0 {
                    is_accending = 2;
                } else if is_accending == 1 {
                    is_safe = false;
                    
                }
                
                let difference_between = previous - current;
                if difference_between < 1 || difference_between > 3 {
                    is_safe = false;
                    
                }
            } else if current == previous {
                is_safe = false;
                
            }
        }
        if is_safe {
            safe_reports += 1;
        } else {
            unsafe_reports += 1;
        }
    }
    assert!(safe_reports + unsafe_reports == reports.len());
    safe_reports
}

fn part_two(input: &str) -> u32 {
    let reports: Vec<Vec<u32>> = input.lines().map(|line| line.split(" ").map(|s| s.parse().unwrap()).collect()).collect();
    let mut safe_reports = 0;
    for report in reports.iter() {
        
        if proccess_record_two(report ,0) {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn proccess_record_two(report: &Vec<u32>, depth: u32) -> bool {
    let mut is_safe = true; 
    if depth > 1 {
        is_safe = false;
        return is_safe;
    }
    let mut is_accending = 0; // 0 - not accending, 1 - accending, 2 - decending
    for i in 1..report.len() {
        let current = report[i];
        let previous = report[i - 1];
        if current > previous {
            if is_accending == 0 {
                is_accending = 1;
            } else if is_accending == 2 {
                is_safe = false;
                break;
            }

            let difference_between = current - previous;
            if difference_between < 1 || difference_between > 3 {
                is_safe = false;
                break;
            } 
        } else if current < previous {
            if is_accending == 0 {
                is_accending = 2;
            } else if is_accending == 1 {
                is_safe = false;
                break;
            }
        
            let difference_between = previous - current;
            if difference_between < 1 || difference_between > 3 {
                is_safe = false;
                break;
            }
        } else if current == previous {
            is_safe = false;
            break;
        }
    }
    if is_safe {
        return is_safe;
    }
    // try again but each time remove one element until it is safe
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);
        let new_is_safe = proccess_record_two(&new_report, depth + 1);
        if new_is_safe {
            return new_is_safe;
        }
    }
    is_safe
}
