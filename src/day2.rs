pub fn day2(input_lines: &[String]) -> (u64, u64) {
    let inputs: Vec<String> = input_lines.iter().map(|line| line.parse::<String>().expect("Failed to parse input")).collect();

    println!("{:?}", inputs);

    let mut total_count_part1 = 0;
    let mut total_count_part2 = 0;
    
    for input in &inputs {

        let split1: Vec<&str> = input.split('-').collect();

        let lower_bound = split1[0].parse::<usize>().expect("Didn't work");

        let split2: Vec<&str> = split1[1].split(' ').collect();

        let upper_bound = split2[0].parse::<usize>().expect("Didn't work");
        let password = String::from(split2[2]);

        let split3: Vec<&str> = split2[1].split(':').collect();

        let character = split3[0].parse::<char>().expect("Didn't work");

        let input_struct = build_input(lower_bound,upper_bound,character,password);

        let mut count_part1 = 0;
        
        for letter in input_struct.password.chars(){
            if letter == input_struct.character {
                count_part1 = count_part1 + 1;
            }
        }

        let char_vec: Vec<char> = input_struct.password.chars().collect();

        if (char_vec[lower_bound-1] == input_struct.character) ^ (char_vec[upper_bound-1] == input_struct.character) {
            total_count_part2 = total_count_part2 + 1;
        }
        
        if count_part1 >= input_struct.lower_bound && count_part1 <= input_struct.upper_bound {
            total_count_part1 = total_count_part1 + 1;
        }

    }

    (total_count_part1,total_count_part2)
}

struct Input {
    lower_bound: usize,
    upper_bound: usize,
    character: char,
    password: String,
}

fn build_input(lower_bound: usize, upper_bound: usize, character: char, password: String) -> Input {
    Input{
        lower_bound,
        upper_bound,
        character,
        password
    }
}