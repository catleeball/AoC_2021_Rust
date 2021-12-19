use itertools::Itertools;
use crate::utils::file_to_str;

pub fn run_day_one() {
    println!("Day One:");
    println!("  Part One (mut): {}", part_one().to_string());
    println!("  Part One (imm): {}", part_one_immutable().to_string());
    println!("  Part Two (imm): {}", part_two().to_string());
}

fn part_one() -> u16 {
    let mut increases: u16 = 0;
    let mut prev: u16 = 0;
    let mut first_loop: bool = true;

    let lines: String = file_to_str("./src/day_one.input.txt");

    for line in lines.lines() {
        let depth: u16 = line.parse().unwrap_or(0);
        if first_loop {
            first_loop = false;
            prev = depth;
            continue;
        }
        if depth > prev {
            increases += 1;
        }
        prev = depth;
    }

    increases
}

fn part_one_immutable() -> usize {
    file_to_str("./src/day_one.input.txt")
        .lines()
        .map(|line| line.parse::<u16>().unwrap_or(0))
        .tuple_windows::<(u16, u16)>()
        .filter(|tup| tup.0 < tup.1)
        .count()
}

fn part_two() -> usize {
    file_to_str("./src/day_one.input.txt")
        .lines()
        .map(|line| line.parse::<u16>().unwrap_or(0))
        .tuple_windows::<(u16, u16, u16)>()
        .map(|triplet| triplet.0 + triplet.1 + triplet.2 )
        .tuple_windows::<(u16, u16)>()
        .filter(|pair| pair.0 < pair.1)
        .count()
}