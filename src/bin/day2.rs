use std::collections::HashMap;

use advent_of_code_2023::read_lines;

fn main() {
    println!("Hello World! Day2");

    println!("Part 1:");
    part1();

    println!("Part 2:");
    part2();
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn part2() {
    if let Ok(lines) = read_lines("./input/day2") {
        let mut sum: u32 = 0;
        for line in lines {
            if let Ok(l) = line {
                let game_info = parse_line(&l);
                let mut maxs = HashMap::from([
                    (DiceColor::RED, 0),
                    (DiceColor::GREEN, 0),
                    (DiceColor::BLUE, 0),
                ]);
                for game in game_info.games {
                    for col in [DiceColor::RED, DiceColor::GREEN, DiceColor::BLUE] {
                        if let (Some(&score), Some(&max)) = (game.get(&col), maxs.get(&col)) {
                            if score > max {
                                maxs.insert(col, score);
                            }
                        }
                    }
                }
                let prod = maxs.iter().fold(1, |cur_prod, (_, &max)| cur_prod * max);
                sum += prod;
            }
        }
        println!("Sum was: {}", sum);
    }
}

fn part1() {
    if let Ok(lines) = read_lines("./input/day2") {
        let mut sum: u32 = 0;
        for line in lines {
            if let Ok(l) = line {
                let game_info = parse_line(&l);
                // println!("{:?}", game_info);
                let mut is_possible = true;
                for game in game_info.games {
                    for (col, max) in [
                        (DiceColor::RED, MAX_RED),
                        (DiceColor::GREEN, MAX_GREEN),
                        (DiceColor::BLUE, MAX_BLUE),
                    ] {
                        if let Some(&score) = game.get(&col) {
                            if score > max {
                                is_possible = false;
                            }
                        }
                    }
                }
                if is_possible {
                    sum += game_info.game_id;
                }
            }
        }
        println!("Sum was: {}", sum);
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum DiceColor {
    RED,
    GREEN,
    BLUE,
}
#[derive(Debug)]
struct GameInfo {
    game_id: u32,
    games: Vec<HashMap<DiceColor, u32>>,
}

fn parse_line(line: &str) -> GameInfo {
    let mut game_id: Option<u32> = None;
    let mut games: Option<Vec<HashMap<DiceColor, u32>>> = None;

    let parts = line.split(": ").collect::<Vec<&str>>();
    if let Some(&game_part) = parts.get(0) {
        if let Some(&id_str) = game_part.split(" ").collect::<Vec<&str>>().get(1) {
            if let Ok(id) = id_str.parse::<u32>() {
                game_id = Some(id);
            }
        }
    }

    if let Some(&results_part) = parts.get(1) {
        games = Some(Vec::new());
        for game_res in results_part.split("; ") {
            let mut res: HashMap<DiceColor, u32> = HashMap::new();
            for color_res in game_res.split(", ") {
                let score_color = color_res.split(" ").collect::<Vec<&str>>();
                let mut color: Option<DiceColor> = None;
                let mut score: Option<u32> = None;
                if let Some(&color_str) = score_color.get(1) {
                    color = match color_str {
                        "blue" => Some(DiceColor::BLUE),
                        "red" => Some(DiceColor::RED),
                        "green" => Some(DiceColor::GREEN),
                        _ => panic!("could not parse the color"),
                    };
                }
                if let Some(&score_str) = score_color.get(0) {
                    if let Ok(i) = score_str.parse::<u32>() {
                        score = Some(i);
                    }
                }
                match (color, score) {
                    (Some(c), Some(s)) => res.insert(c, s),
                    _ => panic!("could not get score and color"),
                };
            }
            match &mut games {
                Some(g) => g.push(res),
                _ => panic!("games got none"),
            }
        }
    }

    match (game_id, games) {
        (Some(id), Some(g)) => GameInfo {
            game_id: id,
            games: g,
        },
        p => panic!("could not find game_id or games: {:?}", p),
    }
}
