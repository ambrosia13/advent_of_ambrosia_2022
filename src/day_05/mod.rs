fn is_valid_column_index(index: i32) -> bool {
    (index - 1) % 4 == 0
}

fn get_column_index(index_in_line: i32) -> i32 {
    (index_in_line - 1) / 4
}

fn get_column_count(input: &str) -> (usize, usize) {
    let mut index: usize = 0;
    let mut count: usize = 0;

    for (i, line) in input.lines().enumerate() {
        let mut chars = line.chars();
        if chars.nth(1).unwrap().is_ascii_digit() {
            let last = chars.last().expect("couldn't get last character in line");
            let last = last
                .to_digit(10)
                .unwrap_or_else(|| panic!("couldn't convert last char to digit: '{last}'"));

            count = last as usize;
            index = i;
            break;
        }
    }

    (index, count)
}

fn parse_to_lists(input: &str) -> Vec<Vec<char>> {
    let mut lines = input.lines();
    let mut lists = Vec::new();

    let (list_end_index, column_count) = get_column_count(input);

    for _ in 0..column_count {
        lists.push(Vec::new());
    }

    let mut i = 0;
    loop {
        let line = lines.next();

        if i == list_end_index {
            break;
        }

        let line = line.unwrap();

        for (i, ch) in line.chars().enumerate() {
            if ch.is_alphabetic() && is_valid_column_index(i as i32) {
                lists[get_column_index(i as i32) as usize].push(ch);
            }
        }

        i += 1;
    }

    lists
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from: u32,
    to: u32,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    // Get the instructions only of the string
    let input = input
        .split_once("\n\n")
        .expect("input failed to split when parsing instructions")
        .1;

    let mut instructions = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        let mut words = line.split_ascii_whitespace();

        let quantity: usize = words.nth(1).unwrap().parse().unwrap();
        let from: u32 = words.nth(1).unwrap().parse::<u32>().unwrap() - 1;
        let to: u32 = words.nth(1).unwrap().parse::<u32>().unwrap() - 1;

        instructions.push(Instruction { quantity, from, to });
    }

    instructions
}

fn execute_instruction(lists: &mut [Vec<char>], instruction: &Instruction) {
    let from: usize = instruction.from as usize;
    let to: usize = instruction.to as usize;

    let mut chars = Vec::new();

    for _ in 0..instruction.quantity {
        chars.push(lists[from].remove(0));
    }

    for ch in chars {
        lists[to].insert(0, ch);
    }
}

pub fn part_one(input: &str) -> String {
    let mut lists = parse_to_lists(input);
    let instructions = parse_instructions(input);

    for instruction in &instructions {
        execute_instruction(&mut lists, instruction);
    }

    let mut stack_tops = String::new();

    for list in lists {
        stack_tops.push(list[0]);
    }

    stack_tops
}

pub fn test() {
    println!("Day 5:");

    let input = include_str!("input.txt").replace("\r\n", "\n");
    let input = input.as_str();

    println!("\tPart one: {}", part_one(input));
}
