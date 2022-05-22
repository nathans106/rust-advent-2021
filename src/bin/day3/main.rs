use diagnostics::Diagnostics;
use rust_advent_2021::{input, day::Day, problem::Problem};

mod diagnostics;

struct PowerConsumptionProblem {}
impl Problem<Vec<String>> for PowerConsumptionProblem {
    fn solve(&mut self, input: &Vec<String>) -> Result<i32, rust_advent_2021::problem::ProblemError> {
        let diagnostics = Diagnostics::from_report(input);
        Ok(diagnostics.power_consumption())
    }
}

struct LifeSupportProblem {}
impl Problem<Vec<String>> for LifeSupportProblem {
    fn solve(&mut self, input: &Vec<String>) -> Result<i32, rust_advent_2021::problem::ProblemError> {
        let diagnostics = Diagnostics::from_report(input);
        Ok(diagnostics.life_support_rating())
    }
}


fn main() {
    let mut day = Day::new(3, input::parse_strs(&3));
    day.add_problem(PowerConsumptionProblem{});
    day.add_problem(LifeSupportProblem{});
    day.solve();
}