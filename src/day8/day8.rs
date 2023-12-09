use std::fs;
use std::collections::HashMap;
use num_integer::lcm;

fn day8_part2(lines: &Vec<&str>) -> u128 {

    let directions: String = lines[0].to_string();
    let nodes: Vec<&str> = lines[1].split("\n").collect();
    let mut node_mapping: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut starting_nodes: Vec<&str> = Vec::new();

    // map out the nodes and their destinations
    for node in nodes {
        let node_str_split: Vec<&str> = node.split(" = ").map(|s| s.trim()).collect();
        let node: &str = node_str_split[0]; 
        let destinations_split: Vec<&str> = node_str_split[1]
                                            .trim_matches(|c| c == '(' || c == ')')
                                            .split(',')
                                            .map(|s| s.trim())
                                            .collect();
        node_mapping.insert(node, (destinations_split[0], destinations_split[1]));
        
        // gather the starting nodes as we go thru input.
        if let Some(last_char) = node.chars().last() {
            if last_char == 'A' {
                starting_nodes.push(node)
            }
        }
    }

    // LCM works based on the input (starting node reaches same node ending in 'Z' every X iterations, always)
    // identify what iteration each starting node reaches a node ending in 'Z' and take LCM.
    let mut end_in_z_counts: Vec<u128> = Vec::new();
    for &node in &starting_nodes {
        let mut count: u128 = 0;
        let mut cur_node: &str = node;
        
        // loop until we end up on a node that ends w/ 'Z'
        while !cur_node.ends_with('Z') {
            for char in directions.chars() {
                let next_options: (&str, &str) = node_mapping[cur_node];
                match char {
                    'L' => cur_node = next_options.0,
                    'R' => cur_node = next_options.1,
                    _ => println!("Should not be here.")
                }
                
                count += 1;
                if cur_node.ends_with('Z') {
                    end_in_z_counts.push(count);
                    }
                }
            }
    }
    
    // take the LCM of the counts.
    let result: u128 = end_in_z_counts.iter().fold(1, |acc, &num| lcm(acc, num));
    return result

}

fn day8_part1(lines: &Vec<&str>) -> u32 {

    let directions: String = lines[0].to_string();
    let nodes: Vec<&str> = lines[1].split("\n").collect();
    let mut node_mapping: HashMap<&str, (&str, &str)> = HashMap::new();

    // map out the nodes and their potential destination
    for node in nodes {
        let node_str_split: Vec<&str> = node.split(" = ").map(|s| s.trim()).collect();
        let node: &str = node_str_split[0]; 
        let destinations_split: Vec<&str> = node_str_split[1]
                                            .trim_matches(|c| c == '(' || c == ')')
                                            .split(',')
                                            .map(|s| s.trim())
                                            .collect();
        node_mapping.insert(node, (destinations_split[0], destinations_split[1]));
    }

    // starting from 'AAA', loop infinitely until you land on 'ZZZ', return number of iterations taken to get there.
    let mut cur_node: &str = "AAA";
    let mut count: u32 = 0;
    while cur_node != "ZZZ" {
        for char in directions.chars() {
            let next_options: (&str, &str) = node_mapping[cur_node];
            match char {
                'L' => cur_node = next_options.0,
                'R' => cur_node = next_options.1,
                _ => println!("Should not be here.")
            }
            count += 1;
        }        
    }

    return count

}

fn main() {
    
    let file_path = "data/day_8_input.txt";
    let input_data_str = fs::read_to_string(file_path).expect("file_path doesn't exist.");
    let lines: Vec<&str> = input_data_str.trim().split("\n\n").collect();
    let part1_answer = day8_part1(&lines);
    let part2_answer = day8_part2(&lines);

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
    
}

