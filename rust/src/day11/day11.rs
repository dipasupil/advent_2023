use std::fs;


fn calc_distances_sum(universe: &Vec<Vec<char>>, has_galaxy_rows: &Vec<bool>, has_galaxy_cols: &Vec<bool>, multiplier: usize) -> u64 {

    // get galaxy coordinates
    let mut galaxy_locations: Vec<(usize, usize)> = Vec::new();
    for (row_idx, row) in universe.iter().enumerate() {
        for (char_idx, char) in row.iter().enumerate() {
            match char {
                '#' => { galaxy_locations.push((row_idx, char_idx)); }
                _ => {}
            }
        }
    }

    // note which rows/cols do not have galaxies in them.
    let mut no_galaxy_cols: Vec<usize> = Vec::new();
    let mut no_galaxy_rows: Vec<usize> = Vec::new();
    for (idx, has_galaxy) in has_galaxy_rows.iter().enumerate() {
        if !has_galaxy {
            no_galaxy_rows.push(idx);
        }
    }

    for (idx, has_galaxy) in has_galaxy_cols.iter().enumerate() {
        if !has_galaxy {
            no_galaxy_cols.push(idx);
        }
    }

    // calc distance between each galaxy pair, accounting for any intersections w/ rows/cols w/o galaxies.
    let mut total_sum: usize = 0;
    for (idx, &(x1, y1)) in galaxy_locations.iter().enumerate() {
        for &(x2, y2) in galaxy_locations.iter().skip(idx + 1) {
            let mut row_intersect_count: usize = 0;
            let mut col_intersect_count: usize = 0;

            // check if row coord intersects w/ any no-galaxy rows.
            for row in &no_galaxy_rows {
                if row > &x1.min(x2) && row < &x1.max(x2) {
                    row_intersect_count += multiplier - 1
                }
            }

            // check if col coord intersects w/ any no-galaxy rows.
            for col in &no_galaxy_cols {
                if col > &y1.min(y2) && col < &y1.max(y2) {
                    col_intersect_count += multiplier - 1
                }
            }

            // calc distance sum for current pair
            total_sum += (x2 as isize - x1 as isize).abs() as usize + (y2 as isize - y1 as isize).abs() as usize + row_intersect_count + col_intersect_count;
        }
    }

    return total_sum as u64
}

fn day9(lines: &Vec<&str>) -> (u64, u64) {

    // go through input and create universe as 2D char array.
    let mut has_galaxy_rows: Vec<bool> = vec![false; lines.len()];
    let mut has_galaxy_cols: Vec<bool> = vec![false; lines[0].len()];
    let mut universe: Vec<Vec<char>> = Vec::new();

    // note which rows have galaxies
    for (row_idx, &line) in lines.iter().enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    has_galaxy_rows[row_idx] = true;
                    has_galaxy_cols[col_idx] = true;
                }
                _ => {}
            }
        }
        universe.push(line.chars().collect());
    }

    // get answers to both parts
    let part1_answer: u64 = calc_distances_sum(&universe, &has_galaxy_rows, &has_galaxy_cols, 2);
    let part2_answer: u64 = calc_distances_sum(&universe, &has_galaxy_rows, &has_galaxy_cols, 1000000);

    return (part1_answer, part2_answer)

}

fn main() {
    
    let file_path: &str = "data/day_11_input.txt";
    let input_data_str: String = fs::read_to_string(file_path).expect("file_path doesn't exist.");
    let lines: Vec<&str> = input_data_str.trim().split("\n").collect();
    let (part1_answer, part2_answer): (u64, u64) = day9(&lines);

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
    
}

