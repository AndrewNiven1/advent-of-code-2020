use itertools::Itertools;

static TARGET: u64 = 2020;

// It's not a remotely performant solution, but it's pretty.

pub fn day1(input_lines: &[String]) -> (u64, u64) {
    let expenses: Vec<u64> = input_lines.iter().map(|line| line.parse::<u64>().expect("Failed to parse input")).collect();
    
    let part1 = find_summing_combination_product(&expenses,2).expect("Failed to solve part 1");
    let part2 = find_summing_combination_product(&expenses,3).expect("Failed to solve part 2");


    (part1, part2)
}

fn find_summing_combination_product(input_vector: &[u64], entry_count: usize) -> Option<u64> {

    let answer = input_vector
                    .iter()
                    .combinations(entry_count)
                    .find(|combo| combo.iter().copied().sum::<u64>() == TARGET)
                    .and_then(|combo| Some(combo.iter().copied().product()));

    answer
                        
}  
