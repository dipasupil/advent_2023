use std::fs;
use std::collections::HashMap;

fn find_num_part2(line: &str) -> i32 {
 
    // get the game set and split out accordingly
    let line_colon_split: Vec<&str> = line.split(":").collect();
    let game_set: Vec<&str> = line_colon_split.last()
                                            .expect("No Games Listed")
                                            .trim()
                                            .split(";")
                                            .collect();

    // find the highest ball count per ball color across all games in the set (Techncially don't need to split games up)
    let mut ball_min_map = HashMap::new();
    ball_min_map.insert("red", 0);
    ball_min_map.insert("green", 0);
    ball_min_map.insert("blue", 0);

    for game in &game_set {
        let balls = game.trim()
                        .split(",");
        
        for ball in balls {
            for color in ["green", "red", "blue"] {
                if ball.contains(color) {
                    let ball_num: i32 = ball.trim()
                                            .split_whitespace()
                                            .next()
                                            .expect("Empty array")
                                            .trim()
                                            .parse()
                                            .expect("Not a number");
                    
                    ball_min_map.insert(color, ball_min_map[color].max(ball_num));
                    }
                }
            }
        }

    // return the max ball counts from the game set multiplied together
    let power_sum: i32 = ball_min_map["green"] * ball_min_map["red"] * ball_min_map["blue"];
    return power_sum

}


fn find_num_part1(line: &str) -> i32 {

    // get the game_id and the game set (split out string properly)
    let line_colon_split: Vec<&str> = line.split(":").collect();
    let game_id: i32 = line_colon_split.get(0)
                                        .expect("No Game ID Listed")
                                        .trim()
                                        .split_whitespace()
                                        .last()
                                        .expect("No Game ID Found")
                                        .parse()
                                        .expect("Failed to parse Game ID");

    let game_set: Vec<&str> = line_colon_split.get(1)
                                            .expect("No Games Listed")
                                            .trim()
                                            .split(";")
                                            .collect();
    
    // game set is impossible if in any game, any ball number is above its stated max (Techncially don't need to split games up)
    let mut possible: bool = true;
    let mut ball_max_map = HashMap::new();
    ball_max_map.insert("red", 12);
    ball_max_map.insert("green", 13);
    ball_max_map.insert("blue", 14);
    
    for game in game_set {
        let balls = game.trim()
                        .split(",");
        
        // check that the each ball's count is smaller than its max. If not, game is impossible. 
        for ball in balls {
            for color in ["green", "red", "blue"] {
                if ball.contains(color) {
                    let ball_num: i32 = ball.trim()
                                            .split_whitespace()
                                            .next()
                                            .expect("Empty array")
                                            .trim()
                                            .parse()
                                            .expect("Not a stringifiednumber");
                                        
                    if ball_num > *ball_max_map.get(color).expect("Color not in map") {
                        possible = false
                    }
                }
            }
        }
    }

    // if all games were possible, send the game_id back -- otherwise, send 0
    if possible {
        return game_id;
    }
    else {
        return 0;
    }
}


fn day2(lines: Vec<&str>) -> (i32, i32) {

    let mut sum_part1: i32 = 0;
    let mut sum_part2: i32 = 0;

    for line in lines {
        sum_part1 += find_num_part1(&line);
        sum_part2 += find_num_part2(&line);
    }

    return (sum_part1, sum_part2);
}


fn main() {
    
    let file_path = "data/day_2_input.txt";
    let contents = fs::read_to_string(&file_path)
                    .expect("Cannot read file.");

    let lines: Vec<&str> = contents.trim().split("\n").collect();
    
    let (sum_part1, sum_part2) = day2(lines.clone());
    
    println!("Day 1 Part 1 Sum: {}", sum_part1);
    println!("Day 2 Part 2 Sum: {}", sum_part2);

}

