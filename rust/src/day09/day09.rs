use std::fs;

fn are_all_zeros(vector: &Vec<i32>) -> bool {
    vector.iter().all(|&x| x == 0)
}

fn get_next_seq(cur_seq: &Vec<i32>) -> Vec<i32> {
    let mut next_seq: Vec<i32> = Vec::new();
    for i in 0..cur_seq.len() - 1 {
        next_seq.push(cur_seq[i + 1] - cur_seq[i]);
    }

    return next_seq
}

fn part1(mut all_seqs: Vec<Vec<i32>>) -> i32 {

    for i in (1..all_seqs.len()).rev() {
            let last_val = all_seqs[i].last().cloned().unwrap();
            let prior_last_val = all_seqs[i-1].last().cloned().unwrap();
            all_seqs[i-1].push(last_val + prior_last_val);
        }
    
    return *all_seqs[0].last().unwrap();
}

fn part2(mut all_seqs: Vec<Vec<i32>>) -> i32 {

    for i in (1..all_seqs.len()).rev() {
        let first_val: i32 = all_seqs[i].first().cloned().unwrap();
        let prior_first_val: i32 = all_seqs[i-1].first().cloned().unwrap();
        all_seqs[i-1].insert(0, prior_first_val - first_val);
    }

    return *all_seqs[0].first().unwrap();

}


fn day9(lines: &Vec<&str>) -> (i32, i32) {

    let mut part1_answer: i32 = 0;
    let mut part2_answer: i32 = 0;

    // go through each history and add its answer to total answers
    for &line in lines {
        let mut all_seqs: Vec<Vec<i32>> = Vec::new();
        let history: Vec<i32> = line.split_whitespace().map(|s: &str| s.parse().unwrap()).collect();
        all_seqs.push(history.clone());
        let mut cur_seq: Vec<i32> = history;

        // create new sequences until sequences becomes all 0s
        while !are_all_zeros(&cur_seq) {
            let next_seq: Vec<i32> = get_next_seq(&cur_seq);
            all_seqs.push(next_seq.clone());
            cur_seq = next_seq;
        }
        
        // calculate answers to both part1 and part2
        part1_answer += part1(all_seqs.clone());
        part2_answer += part2(all_seqs.clone());
    
    }

    return (part1_answer, part2_answer)

}

fn main() {
    
    let file_path: &str = "data/day_9_input.txt";
    let input_data_str: String = fs::read_to_string(file_path).expect("file_path doesn't exist.");
    let lines: Vec<&str> = input_data_str.trim().split("\n").collect();
    let (part1_answer, part2_answer): (i32, i32) = day9(&lines);

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
    
}

