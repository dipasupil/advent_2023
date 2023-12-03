use std::fs;
use std::collections::HashMap;
use regex::Regex;


fn get_number_to_column_idx_sets(row: &str, numbers: Vec<i32>) -> HashMap<i32, Vec<Vec<i32>>> {
    let mut number_to_column_idxs_sets: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
    let mut skipped: i32 = 0;

    for number in numbers {
        let number_str = number.to_string();
    }
    
    return number_to_column_idxs_sets
}
fn day3_part1(lines: Vec<&str>) -> i32 {

    let mut total_sum: i32 = 0;
    let re_pattern = Regex::new(r"\d+").unwrap();

    for (row_idx, row) in lines.iter().enumerate() {
        let numbers_in_row: Vec<i32> = re_pattern.find_iter(row)
                                .map(|m| m.as_str()
                                .parse::<i32>().unwrap())
                                .collect();
        let mut number_to_column_idxs_sets: HashMap<i32, Vec<Vec<i32>>> = get_number_to_column_idx_sets(row, numbers_in_row);
        break
    }

    return total_sum
}


fn main() {
    
    let file_path = "data/day_3_input.txt";
    let contents = fs::read_to_string(&file_path)
                    .expect("Cannot read file.");

    let lines: Vec<&str> = contents.trim().split("\n").collect();
    let sum_part1 = day3_part1(lines.clone());
    println!("{}", sum_part1)
    

}

