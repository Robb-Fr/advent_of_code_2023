use std::collections::HashMap;

use advent_of_code_2023::read_lines;

fn main() {
    println!("Hello, world! Day1");

    println!("Part 1:");
    part1();

    println!("Part 2:");
    part2();
}

fn part2() {
    if let Ok(lines) = read_lines("./input/day1") {
        let mut sum: u32 = 0;
        let patterns_map = HashMap::from([
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);
        let patterns: Vec<&&str> = patterns_map.keys().collect();
        for line in lines {
            if let Ok(l) = line {
                let mut index_match: Vec<(usize, &str)> = Vec::new();
                let l_rev = l.chars().rev().collect::<String>();
                for &&pat in patterns.iter() {
                    if let Some(found) = l.find(pat) {
                        index_match.push((found, pat));
                    }
                    // must check if we find but backward
                    if let Some(found) = l_rev.find(&pat.chars().rev().collect::<String>()) {
                        index_match.push((l.len() - found, pat));
                    }
                }
                let min_option =
                    index_match
                        .iter()
                        .fold(None, |cur_min: Option<(usize, &str)>, &val| match cur_min {
                            Some(cur_min_val) => {
                                if val.0 < cur_min_val.0 {
                                    Some(val)
                                } else {
                                    cur_min
                                }
                            }
                            None => Some(val),
                        });
                if let Some(min) = min_option {
                    if let Some(&left_val) = patterns_map.get(min.1) {
                        sum += left_val * 10;
                    }
                }
                let max_option =
                    index_match
                        .iter()
                        .fold(None, |cur_max: Option<(usize, &str)>, &val| match cur_max {
                            Some(cur_max_val) => {
                                if val.0 > cur_max_val.0 {
                                    Some(val)
                                } else {
                                    cur_max
                                }
                            }
                            None => Some(val),
                        });
                if let Some(max) = max_option {
                    if let Some(&right_val) = patterns_map.get(max.1) {
                        sum += right_val;
                    }
                }
            }
        }
        println!("Sum was: {}", sum);
    }
}

fn part1() {
    if let Ok(lines) = read_lines("./input/day1") {
        let mut sum: u32 = 0;
        for line in lines {
            if let Ok(l) = line {
                for c in l.chars() {
                    if let Some(i) = c.to_digit(10) {
                        sum += 10 * i;
                        break;
                    }
                }
                for c in l.chars().rev() {
                    if let Some(i) = c.to_digit(10) {
                        sum += i;
                        break;
                    }
                }
            }
        }
        println!("Sum was: {}", sum);
    }
}
