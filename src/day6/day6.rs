use std::fs;

fn day6(lines: &Vec<&str>) -> (u64, u64) {

    ///// part 1
    let times: Vec<u64> = lines[0].split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    let distances: Vec<u64> = lines[1].split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();

    // for each race, find number of ways to finish; multiply all number of ways together for answer. 
    let mut total_part1 = 1;
    for idx in 0..times.len() {
        let mut num_ways = 0;
        for second in 0..times[idx] {
            if (times[idx] - second) * second >= distances[idx] {
                num_ways += 1
            }
        }
        total_part1 *= num_ways;
    }

    ///// part 2
    let total_time: u64 = lines[0].split_whitespace().skip(1).collect::<String>().parse().unwrap();
    let total_distance: u64 = lines[1].split_whitespace().skip(1).collect::<String>().parse().unwrap();
    
    // for each race, find number of ways to finish.
    let mut total_part2 = 0;
    for second in 0..total_time {
        if (total_time - second) * second >= total_distance{
            total_part2 += 1
        }
    }

    return (total_part1, total_part2)
}

fn main() {
    
    let file_path = "data/day_6_input.txt";
    let input_data_str = fs::read_to_string(file_path).expect("Cannot read file.");
    let lines: Vec<&str> = input_data_str.trim().split("\n").collect();
    let (part1_answer, part2_answer) = day6(&lines);
    println!("{}", part1_answer);
    println!("{}", part2_answer);

    
}

