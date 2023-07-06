// Helper function for part 1 and 2
// Returns the count of all the elves and their calorie counts
fn get_calorie_counts(input: &str) -> Vec<(u8, u32)> {
    let lines = input.lines();

    let mut current_elf_index = 1;
    let mut current_elf_calories = 0;

    let mut calorie_counts: Vec<(u8, u32)> = Vec::new();

    for line in lines {
        let parsed_line = line.trim().parse::<u32>();

        if let Ok(calorie_count) = parsed_line {
            current_elf_calories += calorie_count;
        } else {
            calorie_counts.push((current_elf_index, current_elf_calories));

            current_elf_index += 1;
            current_elf_calories = 0;
        }
    }

    calorie_counts
}

// Returns the elf with the highest calorie count.
//
// Tuple result:
// 0 - the index of the elf carrying the most calories
// 1 - how many calories that elf was carrying
pub fn part_one(input: &str) -> (u8, u32) {
    let calorie_counts = get_calorie_counts(input);

    let mut result: (u8, u32) = (0, 0);

    for element in &calorie_counts {
        if element.1 > result.1 {
            (result.0, result.1) = (element.0, element.1);
        }
    }

    result
}

// Helper function for part two
fn remove_by_elf_id(calorie_counts: &mut Vec<(u8, u32)>, element: &(u8, u32)) {
    for index in (0..calorie_counts.len()).rev() {
        if calorie_counts[index].0 == element.0 {
            calorie_counts.remove(index);
        }
    }
}

pub fn part_two(input: &str) -> [(u8, u32); 3] {
    let mut calorie_counts = get_calorie_counts(input);
    let mut top_three: [(u8, u32); 3] = [(0, 0); 3];

    for i in 0..3 {
        let mut greatest: (u8, u32) = (0, 0);

        for element in &calorie_counts {
            if element.1 > greatest.1 {
                (greatest.0, greatest.1) = (element.0, element.1);
            }
        }

        remove_by_elf_id(&mut calorie_counts, &greatest);
        top_three[i] = greatest;
    }

    top_three
}

// Tests part one and two
pub fn test() {
    println!("Day 1:");

    let input = include_str!("input.txt").to_string();

    println!("\tPart one: {:?}", part_one(&input));

    let res = part_two(&input);
    let sum = res[0].1 + res[1].1 + res[2].1;

    println!("\tPart two: {:?}. Sum: {sum}", res);

    println!();
}
