use crate::utils::file_to_str;

pub fn run_day_two() {
    println!("Day Two:");
    println!("  Part One: {}", part_one().to_string());
    println!("  Part Two: {}", part_two().to_string());
}

enum   Direction   { Up, Down, Forward }
struct Position    { x: u32, depth: u32 }
struct Instruction { dir: Direction, qty: u32 }

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        let (dir_str, qty_str) = s.split_once(' ').unwrap_or(("forward", "0"));
        let qty: u32 = qty_str.parse().unwrap_or(0);
        let dir: Direction = match dir_str {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => panic!("Malformed direction: {}", dir_str),
        };
        Instruction { dir, qty }
    }
}

fn part_one() -> u32 {
    let final_position = file_to_str("./src/day_two.input.txt")
        .lines()
        .map(Instruction::from_str)
        .fold(Position{x: 0, depth: 0}, |mut pos, instruct| {
            match instruct.dir {
                Direction::Up      => { pos.depth -= instruct.qty; pos }
                Direction::Down    => { pos.depth += instruct.qty; pos }
                Direction::Forward => { pos.x     += instruct.qty; pos }
            }
        });
    final_position.x * final_position.depth
}

// ========== Part Two ========== //

struct Position2 { x: u32, depth: u32, aim: u32 }

fn part_two() -> u32 {
    let final_position = file_to_str("./src/day_two.input.txt")
        .lines()
        .map(Instruction::from_str)
        .fold(Position2{x: 0, depth: 0, aim: 0}, |mut pos, instruct| {
            match instruct.dir {
                Direction::Up      => { pos.aim -= instruct.qty; pos }
                Direction::Down    => { pos.aim += instruct.qty; pos }
                Direction::Forward => {
                    pos.x     += instruct.qty;
                    pos.depth += (instruct.qty * pos.aim);
                    pos
                }
            }
        });
    final_position.x * final_position.depth
}
