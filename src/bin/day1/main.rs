use rust_advent_2021::{depth, input};

fn main() {
    let depths = input::parse_vec::<depth::Depth>(&1);
    let rate = depth::rate(&depths);
    print!("part 1: {}", rate);
}
