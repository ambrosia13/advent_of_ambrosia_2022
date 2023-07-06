fn parse_section_assignments(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let mut vec = Vec::new();

    for line in input.lines() {
        let mut a = [0; 4];

        let mut number = 0;
        let mut index = 0;
        for ch in line.trim().chars() {
            if ch.is_ascii_digit() {
                number *= 10;
                number += ch.to_digit(10).expect("this is a digit it shouldnt panic") as i32;
            } else {
                index += 1;
                number = 0;
            }

            a[index] = number;
        }

        vec.push(((a[0], a[1]), (a[2], a[3])));
    }

    vec
}

// Validates some assumptions about the input
fn validate(input: &str) {
    let assignments = parse_section_assignments(input);

    for ((a, b), (c, d)) in assignments {
        if a <= b && c <= d {
        } else {
            panic!("{a}, {b}, {c}, {d}");
        }
    }
}

fn range_contains(range_a: &(i32, i32), range_b: &(i32, i32)) -> bool {
    range_a.0 <= range_b.0 && range_b.1 <= range_a.1
}

fn range_overlaps(range_a: &(i32, i32), range_b: &(i32, i32)) -> bool {
    range_a.1 >= range_b.0 && range_b.1 >= range_a.0
}

// Returns true if one of the assignments is completely in the range of the other
// in other words, returns true if the assignment is invalid
fn test_containing_assignments(section_assignment: &((i32, i32), (i32, i32))) -> bool {
    range_contains(&section_assignment.0, &section_assignment.1)
        || range_contains(&section_assignment.1, &section_assignment.0)
}

fn test_overlapping_assignments(section_assignment: &((i32, i32), (i32, i32))) -> bool {
    range_overlaps(&section_assignment.0, &section_assignment.1)
        || range_overlaps(&section_assignment.1, &section_assignment.0)
}

pub fn part_one(input: &str) -> i32 {
    let section_assignments = parse_section_assignments(input);

    let mut total_invalid_assignments = 0;

    for assignment in &section_assignments {
        total_invalid_assignments += test_containing_assignments(assignment) as i32;
    }

    total_invalid_assignments
}

pub fn part_two(input: &str) -> i32 {
    let section_assignments = parse_section_assignments(input);

    let mut total_invalid_assignments = 0;

    for assignment in &section_assignments {
        total_invalid_assignments += test_overlapping_assignments(assignment) as i32;
    }

    total_invalid_assignments
}

pub fn test() {
    println!("Day 4:");

    let input = include_str!("input.txt");

    validate(input);

    println!("\tPart one: {}", part_one(input));
    println!("\tPart two: {}", part_two(input));

    println!();
}
