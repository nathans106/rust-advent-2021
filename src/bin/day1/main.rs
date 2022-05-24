use depth::{RateProblem, SlidingRateProblem};
use rust_advent_2021::{day::Day, input};
mod depth;

fn main() {
    let depths = input::parse_vec::<depth::Depth>(&1);
    let mut day = Day::new(1, depths);
    day.add_problem(RateProblem {});
    day.add_problem(SlidingRateProblem {});
    day.solve();
}
