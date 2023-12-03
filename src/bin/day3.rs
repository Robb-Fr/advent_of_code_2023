use std::fmt::format;

use advent_of_code_2023::read_lines;

fn main() {
    println!("Hello World! Day3");

    println!("Part 1:");
    part1();
}

fn part1() {
    if let Ok(mut lines) = read_lines("./input/day2") {
        let mut sum: u32 = 0;
        let prev_line = lines.next().expect("should have 1 line");
        let cur_line = lines.next().expect("should have 2 lines");
        let next_line = lines.next().expect("should have 3 lines");

        if let (Ok(l), Ok(l_next)) = (prev_line, cur_line) {
            let chars = l.chars().collect::<Vec<char>>();
            let next_chars = l_next.chars().collect::<Vec<char>>();
            for (i, c) in l.char_indices() {
                if c != '.' && !c.is_alphanumeric() {
                    let mut prev_nb = String::from("");
                    let mut i_p = i - 1;
                    while let Some(&prev_ch) = chars.get(i_p) {
                        if prev_ch.is_numeric() {
                            prev_nb = format!("{}{}", prev_ch, prev_nb);
                            i_p -= 1;
                        } else {
                            break;
                        }
                    }
                    if prev_nb != "" {
                        if let Ok(p_nb) = prev_nb.parse::<u32>() {
                            sum += p_nb;
                        }
                    }

                    let mut next_nb = String::from("");
                    let mut i_n = i + 1;
                    while let Some(&next_ch) = chars.get(i_n) {
                        if next_ch.is_numeric() {
                            next_nb = format!("{}{}", next_nb, next_ch);
                            i_n += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        println!("Sum was: {}", sum);
    }
}
