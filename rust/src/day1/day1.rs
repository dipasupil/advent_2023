use std::fs;

fn find_num(line: &str) -> i32 {
    
    // read line and push all digits to found_digits
    let mut found_digits = String::new();
    for char in line.chars() {
        if char.is_digit(10){
            found_digits.push(char)
        }
    }
    
    // take first and last digit and use it to form the number
    let first_char = found_digits.chars().next().unwrap();
    let last_char = found_digits.chars().last().unwrap();
    let number = format!("{}{}", first_char, last_char);
    let number = number.parse().expect("Not a number");
    return number

}

fn day1_part2(lines: Vec<&str>) -> i32 {

    let mut sum = 0;
    for line in lines {

        // Replace the input string's strigified digits to deal w/ overlapping characters
        let replaced_line = line.replace("one", "one1one")
                                .replace("two", "two2two")
                                .replace("three", "three3three")
                                .replace("four", "four4four")
                                .replace("five", "five5five")
                                .replace("six", "six6six")
                                .replace("seven", "seven7seven")
                                .replace("eight", "eight8eight")
                                .replace("nine", "nine9nine");

        sum += find_num(&replaced_line);
    }

    return sum

}

fn day1_part1(lines: Vec<&str>) -> i32 {

    let mut sum: i32 = 0;
    for line in lines {
        sum += find_num(&line);
    }

    return sum;
}

fn main() {
    
    let file_path = "data/day_1_input.txt";
    let contents = fs::read_to_string(&file_path)
                    .expect("Cannot read file.");

    let lines: Vec<&str> = contents.trim().split("\n").collect();
    
    let sum_day1: i32 = day1_part1(lines.clone());
    println!("Day 1 Part 1 Sum: {}", sum_day1);

    let sum_day2: i32 = day1_part2(lines.clone());
    println!("Day 1 Part 2 Sum: {}", sum_day2);

}

