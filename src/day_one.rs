use crate::utils::file_to_str;

pub fn run_day_one() {
    println!("Day One:");
    println!("  Part One: {}", part_one().to_string());
}

fn part_one() -> u16 {
    let mut increases: u16 = 0;
    let mut prev: u16 = 0;
    let mut first_loop: bool = true;

    let lines: String = file_to_str("./src/day_one.input.txt");

    for line in lines.lines() {
        let depth: u16 = line.parse::<u16>().unwrap_or(0);
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

    return increases;
}

fn part_two() {

}
