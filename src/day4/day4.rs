use std::fs;

fn day4_part2(rows: &Vec<&str>) -> i64 {

    // store counts of all scratch cards in an array
    let mut scratchcard_counts = vec![1; rows.len()];

    // go through each scratch card
    for (row_idx, row) in rows.iter().enumerate() {

        let numbers: &str = *row.split(':').collect::<Vec<&str>>().last().expect("No : in string.");
        let numbers_split: Vec<&str> = numbers.split("|").collect();
        let winning_numbers: Vec<i32> = numbers_split[0].trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
        let my_numbers: Vec<i32> = numbers_split[1].trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();

        // get number of matches for row's corresponding card.
        let mut matches = 0;
        for number in my_numbers {
            if winning_numbers.contains(&number) {
                matches += 1
            }
        }
        
        // based on number of matches, increase subsequent cards' counts by (number of copies of current row's card)
        for next_cards_idx in (row_idx+1)..=(row_idx + matches) {
            if next_cards_idx < scratchcard_counts.len() {
                scratchcard_counts[next_cards_idx] += scratchcard_counts[row_idx];
            }
        }
    }

    // return total number of scratchcards

    return scratchcard_counts.iter().sum()
}

fn day4_part1(rows: &Vec<&str>) -> i32 {

    let mut total_sum: i32 = 0;

    // go through all games and calculate total_sum for part 1
    for row in rows {
        let numbers: &str = *row.split(':').collect::<Vec<&str>>().last().expect("No : in string.");
        let numbers_split: Vec<&str> = numbers.split("|").collect();
        let winning_numbers: Vec<i32> = numbers_split[0].trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
        let my_numbers: Vec<i32> = numbers_split[1].trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();

        // calculate game score and add to total_sum
        let mut game_score = 1;
        for number in my_numbers {
            if winning_numbers.contains(&number) {
                game_score *= 2
            }
        }
        total_sum += game_score / 2;
    }

    return total_sum

}

fn main() {
    
    let file_path = "data/day_4_input.txt";
    let contents = fs::read_to_string(&file_path).expect("Cannot read file.");
    let lines: Vec<&str> = contents.trim().split("\n").collect();
    
    let sum_part1 = day4_part1(&lines);
    println!("{}", sum_part1);
    
    let sum_part2 = day4_part2(&lines);
    println!("{}", sum_part2);
    
}

