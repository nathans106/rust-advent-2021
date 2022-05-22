use instruction_parser::parse_instructions;
use navigator::{LinearNavigator, AngularNavigator, Instructions, Navigator};
use rust_advent_2021::input::parse_strs;
use rust_advent_2021::day::Day;
use rust_advent_2021::problem::Problem;

mod navigator;
mod instruction_parser;

impl Problem<Instructions> for LinearNavigator {
    fn solve(&mut self, input: &Instructions) -> Result<i32, rust_advent_2021::problem::ProblemError> {
        let pos = LinearNavigator::calculate_position(&input);
        Ok(pos.depth * pos.horizontal)
    }
}

impl Problem<Instructions> for AngularNavigator {
    fn solve(&mut self, input: &Instructions) -> Result<i32, rust_advent_2021::problem::ProblemError> {
        let pos = AngularNavigator::calculate_position(&input);
        Ok(pos.depth * pos.horizontal)
    }
}

fn main() {
    let day_num = 2;
    let lines = parse_strs(&day_num);
    let instructions = parse_instructions(lines.as_slice());
    let mut day = Day::new(day_num, instructions);
    day.add_problem(LinearNavigator{});
    day.add_problem(AngularNavigator{});

    day.solve();
}