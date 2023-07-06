fn get_priority(character: char) -> i32 {
    if character.is_ascii_lowercase() {
        character as i32 - 96
    } else {
        character as i32 - 38
    }
}

fn get_compartments(rucksack: &str) -> (&str, &str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn find_common_item(compartments: (&str, &str)) -> char {
    let mut common_item: Option<char> = None;

    for ch in compartments.0.chars() {
        for other_ch in compartments.1.chars() {
            if ch == other_ch {
                common_item = Some(ch);
            }
        }
    }

    common_item.expect("No common item was found in the compartments")
}

fn find_common_item_in_rucksacks(rucksacks: (&str, &str, &str)) -> char {
    let mut common_item: Option<char> = None;

    for ch in rucksacks.0.chars() {
        if rucksacks.1.contains(ch) && rucksacks.2.contains(ch) {
            common_item = Some(ch);
        }
    }

    common_item.expect("No common item was found in the three rucksacks")
}

pub fn part_one(input: &str) -> i32 {
    let mut total_priority = 0;

    for line in input.lines() {
        let common_item = find_common_item(get_compartments(line));
        total_priority += get_priority(common_item);
    }

    total_priority
}

pub fn part_two(input: &str) -> i32 {
    let mut total_priority = 0;

    let mut lines = input.lines();

    loop {
        let rucksack_a = lines.next();
        let rucksack_b = lines.next();
        let rucksack_c = lines.next();

        if rucksack_a.is_none() || rucksack_b.is_none() || rucksack_c.is_none() {
            break;
        }

        let rucksack_a = rucksack_a.expect("unable to unwrap next iterator item a");
        let rucksack_b = rucksack_b.expect("unable to unwrap next iterator item b");
        let rucksack_c = rucksack_c.expect("unable to unwrap next iterator item c");

        let common_item = find_common_item_in_rucksacks((rucksack_a, rucksack_b, rucksack_c));
        total_priority += get_priority(common_item);
    }

    total_priority
}

pub fn test() {
    println!("Day 3:");

    let input = include_str!("input.txt");

    println!("\tPart one: {}", part_one(input));
    println!("\tPart two: {}", part_two(input));

    println!()
}
