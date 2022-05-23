use crate::navigator::{Direction, Instruction, Instructions};
use regex::Regex;

pub fn parse_instructions<T: AsRef<str>>(lines: &[T]) -> Instructions {
    lines
        .iter()
        .map(|line| parse_instruction(line.as_ref()))
        .collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let regex = Regex::new(r"(\w+) (\d+)").unwrap();
    let captured = regex.captures(line).unwrap();
    let direction_str = captured.get(1).unwrap().as_str();
    let distance = captured.get(2).unwrap().as_str().parse::<i32>().unwrap();

    let direction = match direction_str {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        &_ => panic!("Unkown direction string"),
    };

    Instruction {
        direction,
        distance,
    }
}
