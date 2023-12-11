use std::fs;
use std::collections::HashMap;


fn get_destination(cur_mapping: &Vec<(u64, u64, u64)>, cur_number: u64) -> u64 {
    
    for tuple in cur_mapping {
        let (destination, start, end) = tuple;
        if cur_number >= *start && cur_number < *end {
            return destination + cur_number - start
        }
    }

    return cur_number
}


fn day5_part1(rows: &Vec<&str>) -> u64 {

    let numbers_str: Vec<&str> = rows[0].split_whitespace().skip(1).collect(); // Skip "seeds:"
    let seeds: Vec<u64> = numbers_str.iter().filter_map(|&s| s.parse().ok()).collect();
    
    // extract the x-to-x maps and store in a hash table based on order they appear in input.
    let mapping_strs = &rows[1..];
    let mut ranges_list: Vec<Vec<(u64, u64, u64)>> = Vec::new(); // dest_start, range_start, range_end

    for (idx, line) in mapping_strs.iter().enumerate() {
        let tuples_as_strings: Vec<&str> = line.split("\n").collect();
        ranges_list.push(Vec::new());
        for tuple in &tuples_as_strings[1..] {
            let numbers: Vec<u64> = tuple.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
            if numbers.len() == 3 {
                ranges_list[idx].push((numbers[0], numbers[1], numbers[1] + numbers[2]));
            }
        }  
    }

    // go thru each seed and calculate the final location number it's mapped to and store in array
    let mut seed_location_mins: Vec<u64> = Vec::new();
    for (seed_idx, seed) in seeds.iter().enumerate() { 
        let mut cur_number: u64 = *seed;
        for map_idx in 0..ranges_list.len() {
            let cur_mapping = &ranges_list[map_idx];
            cur_number = get_destination(&cur_mapping, cur_number);
        }
        seed_location_mins.push(cur_number);
    }
    
    // return the min location number
    let min_value = seed_location_mins.iter().cloned().min();
    match min_value {
        Some(min) => min,
        None => 0,
    }

}

fn main() {
    
    let file_path = "data/day_5_input.txt";
    let contents = fs::read_to_string(file_path).expect("Cannot read file.");
    let lines: Vec<&str> = contents.trim().split("\n\n").collect();
    let part1_answer = day5_part1(&lines);
    println!("{}", part1_answer);
    
}

