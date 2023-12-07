use std::fs;
use std::collections::HashMap;

#[derive(Clone)]
struct Hand {
    bet: i32,
    hand_type: i32,
    hand_as_nums: Vec<u32>,
}

impl Hand {
    fn new(cards: &str, bet: i32, jokers: bool) -> Hand {
        let hand_type = Self::get_hand_type(cards, jokers);
        let hand_as_nums = Self::convert_hand_to_nums(cards, jokers);
        return Hand { bet, hand_type, hand_as_nums }
    }

    fn convert_hand_to_nums(cards: &str, jokers: bool) -> Vec<u32> {
        cards
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if jokers { 1 }
                    else { 11 }
                },
                'T' => 10,
                _ => c.to_digit(10).unwrap() as u32,
            })
            .collect()
    }

    fn get_hand_type(hand: &str, jokers: bool) -> i32 {

        // get char counts of hand and keep track of most frequent non-J character.
        let mut char_counts: HashMap<char, i32> = HashMap::new();
        let mut max_count = 0;
        let mut most_frequent_char = ' ';
        for c in hand.chars() {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;

            // update most frequent non-'J' char and its char_count
            if *count > max_count && c != 'J' {
                max_count = *count;
                most_frequent_char = c;
            }
        }

        // if there are jokers, add number of jokers to most frequent non-J character's count and remove 'J' from map.
        if jokers {
            let j_count = char_counts.get(&'J').copied().unwrap_or(0);
            if let Some(count_most_freq) = char_counts.get_mut(&most_frequent_char) {
                *count_most_freq += j_count;
                char_counts.remove(&'J');
            }
        }

        // return hand_type based on character counts.
        let mut kind3: i32 = 0;
        let mut pairs: i32 = 0;
        for (_, v) in &char_counts {
            match v {
                5 => return 7, // five of a kind
                4 => return 6, // four of a kind
                3 => kind3 += 1,
                2 => pairs += 1,
                _ => continue
            }
        }

        // full house
        if kind3 == 1 && pairs == 1 {
            return 5
        }

        // 3 of a kind
        else if kind3 == 1 {
            return 4
        }

        // 2 pair > 1 pair > 0 pairs
        return pairs + 1

    } 
}


fn order_hands(mut hands: Vec<Hand>) -> Vec<Hand> {

    // sort hands lexicographically
    hands.sort_by(|a, b| {
        a.hand_as_nums.cmp(&b.hand_as_nums)
    });
    
    return hands
}


fn day7(lines: &Vec<&str>) -> (i32, i32) {

    // parse out all hands and store in HashMap based on hand type (7 possibilities)
    let mut hand_types_part1: HashMap<i32, Vec<Hand>> = HashMap::new();
    let mut hand_types_part2: HashMap<i32, Vec<Hand>> = HashMap::new();
    for hand_type in 1..=7 {
       hand_types_part1.insert(hand_type, Vec::new());
       hand_types_part2.insert(hand_type, Vec::new());
    }

    for line in lines {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let cards: String = split_line[0].to_string();
        let bet: i32 = split_line[1].parse().unwrap();
        let hand_part1: Hand = Hand::new(&cards, bet, false);
        let hand_part2: Hand = Hand::new(&cards, bet, true);

        hand_types_part1
            .entry(hand_part1.hand_type)
            .or_insert_with(Vec::new)
            .push(hand_part1);

        hand_types_part2
            .entry(hand_part2.hand_type)
            .or_insert_with(Vec::new)
            .push(hand_part2);
    }

    // order all hands based on their Type then their Rank within that type (gives overall ranking)
    let mut ordered_hands_part1: Vec<Hand> = Vec::new();
    let mut ordered_hands_part2: Vec<Hand> = Vec::new();
    for hand_type in 1..=7 {
        if let Some(hands) = hand_types_part1.get(&hand_type) {
            ordered_hands_part1.extend(order_hands(hands.to_vec()));
        }

        if let Some(hands) = hand_types_part2.get(&hand_type) {
            ordered_hands_part2.extend(order_hands(hands.to_vec()));
        }

        else {
            continue;
        }
    }

    // calculate total score for both parts
    let mut p1_score = 0;
    let mut p2_score = 0;

    for (index, hand) in ordered_hands_part1.iter().enumerate() {
        p1_score += (index as i32 + 1) * hand.bet
    }

    for (index, hand) in ordered_hands_part2.iter().enumerate() {
        p2_score += (index as i32 + 1) * hand.bet
    }

    return (p1_score, p2_score)
}

fn main() {
    
    let file_path = "data/day_7_input.txt";
    let input_data_str = fs::read_to_string(file_path).expect("Cannot read file.");
    let lines: Vec<&str> = input_data_str.trim().split("\n").collect();
    let (part1_answer, part2_answer) = day7(&lines);
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);

    
}

