pub fn day3(input_lines: &[String]) -> (u64, u64) {
    
    let part1 = get_tree_count(input_lines,3,1);

    let shifts_vec: Vec<Shift> = vec![Shift{right_shift: 1, down_shift: 1},
                                    Shift{right_shift: 3, down_shift: 1},
                                    Shift{right_shift: 5, down_shift: 1},
                                    Shift{right_shift: 7, down_shift: 1},
                                    Shift{right_shift: 1, down_shift: 2}];
    let mut part2 = 1;

    for shift in shifts_vec {
        part2 = part2 * get_tree_count(input_lines,shift.right_shift,shift.down_shift)
    }
    (part1,part2)
}

pub fn get_tree_count(input_lines: &[String], right_shift: usize, down_shift: usize) -> u64 {

    let mut index: usize = 0;

    let repeating_unit_length: usize = input_lines[0].chars().count();

    let mut tree_count: u64 = 0;
    let mut line_number: usize = 0;

    while line_number < input_lines.len() {

        let mut input_char_vec : Vec<char> = input_lines[line_number].chars().collect();

        if input_char_vec[index] == '#' {
            tree_count = tree_count + 1;
        }

        index = index + right_shift;

        if index > repeating_unit_length - 1 {
            index = index - repeating_unit_length;
        }

        line_number = line_number + down_shift

    }

    println!("{}",tree_count);

    tree_count

}

struct Shift {
    right_shift: usize,
    down_shift: usize,
}