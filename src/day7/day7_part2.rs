use std::fs;
use std::collections::HashMap;

#[derive(Clone)]
struct Hand {
    bet: i32,
    hand_type: i32,
    hand_as_nums: Vec<u32>
}

impl Hand {
    fn new(cards: &str, bet: i32) -> Hand {
        let hand_type = Self::get_hand_type(cards);
        let hand_as_nums = Self::convert_hand_to_nums(cards);
        return Hand { bet, hand_type, hand_as_nums}
    }

    fn convert_hand_to_nums(cards: &str) -> Vec<u32> {
        cards
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => c.to_digit(10).unwrap() as u32,
            })
            .collect()
    }

    fn get_hand_type(hand: &str) -> i32 {
        let mut char_counts: HashMap<char, i32> = HashMap::new();
        // get char counts
        for c in hand.chars() {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }
    
        let mut kind5: i32 = 0;
        let mut kind4: i32 = 0;
        let mut kind3: i32 = 0;
        let mut pairs: i32 = 0;
    
        // 7 = 5 of kind, 6 = 4 of kind, 5 = full house, 4 = 3 pair, 3 = 2 pair, 2 = 1 pair, 1 = high
        for (_, v) in &char_counts {
            match v {
                5 => kind5 += 1,
                4 => kind4 += 1,
                3 => kind3 += 1,
                2 => pairs += 1,
                _ => continue
            }
        }
    
        // deal w/ Jokers
        if let Some(&count) = &char_counts.get(&'J') {
            
            // if there's one Joker in the hand
            if count == 1 {
                if kind4 == 1 {
                    kind5 += 1;
                    kind4 -= 1;
                }
                else if kind3 == 1 {
                    kind4 += 1;
                    kind3 -= 1;
                }
                else if pairs == 2 {
                    kind3 += 1;
                    pairs -= 1;
                }
                else if pairs == 1 {
                    kind3 += 1;
                    pairs -= 1;
                }
                else if pairs == 0 {
                    pairs += 1
                }
            }
    
            // if there's two Jokers in the hand
            else if count == 2 {
                if kind3 == 1 {
                    kind5 += 1;
                    pairs -= 1;
                }
                
                // if there's another pair, can create a 4 of a kind.
                else if pairs == 2 { 
                    kind4 += 1;
                    pairs -= 1;
                }
                else if pairs == 1{ // if jokers are only pair
                    kind3 += 1;
                    pairs -= 1;
                }
            }
    
            // if there's three Jokers in the hand
            else if count == 3 {
    
                
                if pairs == 1 { 
                    kind5 += 1;
                    pairs -= 1;
                }
    
                // if no other pair, can create a 4 of a kind.
                else { 
                    kind4 += 1;
                    kind3 -= 1;
                }
            }
    
            // if there's four Jokers in the hand
            else if count == 4 {
                kind4 -= 1;
                kind5 += 1;
            }
        } 
    
    
        if kind5 == 1 {
            return 7
        }
        else if kind4 == 1 {
            return 6
        }
        else if kind3 == 1 && pairs == 1 {
            return 5
        }
        else if kind3 == 1 {
            return 4
        }
        else if pairs == 2 {
            return 3
        }
        else if pairs == 1 {
            return 2
        }
    
        return 1
    } 
}


fn order_hands(mut hands: Vec<Hand>) -> Vec<Hand> {

    // sort hands based on tie-breaker system. 1st char > 2nd char > 3rd char etc...
    hands.sort_by(|a, b| {
        a.hand_as_nums.cmp(&b.hand_as_nums) // or customize the comparison logic as needed
    });
    
    return hands
}


fn day7_part2(lines: &Vec<&str>) -> i32 {

    // parse out the hands and bets and store in HashMap based on hand type 
    let mut hand_types: HashMap<i32, Vec<Hand>> = HashMap::new();
    hand_types.insert(1, Vec::new()); // high card
    hand_types.insert(2, Vec::new()); // 1 pair
    hand_types.insert(3, Vec::new()); // 2 pairs
    hand_types.insert(4, Vec::new()); // 3 of a kind
    hand_types.insert(5, Vec::new()); // full house
    hand_types.insert(6, Vec::new()); // 4 of a kind
    hand_types.insert(7, Vec::new()); // 5 of a kind
    
    for line in lines {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let cards: String = split_line[0].to_string();
        let bet: i32 = split_line[1].parse().unwrap();
        
        let hand: Hand = Hand::new(&cards, bet);
        hand_types
            .entry(hand.hand_type)
            .or_insert_with(Vec::new)
            .push(hand);
    }

    // order all hands based on their Type then their Rank within that type (gives overall ranking)
    let mut total_score = 0;
    let mut ordered_hands: Vec<Hand> = Vec::new();
    let keys = [1, 2, 3, 4, 5, 6, 7];
    for key in keys.iter() {
        if let Some(hands) = hand_types.get(key) {
            ordered_hands.extend(order_hands(hands.to_vec()));
        } 
        else {
            continue;
        }
    }

    // calculate total score
    for (index, hand) in ordered_hands.iter().enumerate() {
        total_score += (index as i32 + 1) * hand.bet
    }

    return total_score
}

fn main() {
    
    let file_path = "data/day_7_input.txt";
    let input_data_str = fs::read_to_string(file_path).expect("Cannot read file.");
    let lines: Vec<&str> = input_data_str.trim().split("\n").collect();
    let part2_answer = day7_part2(&lines);
    println!("{}", part2_answer)

    
}

