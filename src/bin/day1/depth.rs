use rust_advent_2021::problem::{Problem, ProblemError};


pub type Depth = i16;
type Input = Vec<Depth>;

pub struct RateProblem;
impl Problem<Input> for RateProblem {
    fn solve(&mut self, depths: &Input) -> Result<i32, ProblemError> {
        Ok(rate(&depths))
    }
}

pub struct SlidingRateProblem;
impl Problem<Input> for SlidingRateProblem {
    fn solve(&mut self, depths: &Input) -> Result<i32, ProblemError> {
        let windowed: Vec<i16> = depths.windows(3).map(|window| window.iter().sum()).collect();
        Ok(rate(&windowed))
    }
}

pub fn rate(depths: &[Depth]) -> i32 {
    depths.windows(2).filter(|&pair| pair[1] > pair[0]).count().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rate(depths: &Vec<Depth>, expected: i32) {
        let mut problem = RateProblem{};
        let result = problem.solve(&depths).unwrap();
        assert_eq!(result, expected); 
    }

    #[test]
    fn rate_zero_inputs() {
        let depths = vec![];
        test_rate(&depths, 0)
    }

    #[test]
    fn rate_one_input() {
        test_rate(&vec![8], 0)
    }

    #[test]
    fn rate_zero_increases() {
        test_rate(&vec![8, 6, 4], 0)
    }

    #[test]
    fn rate_one_increase() {
        test_rate(&vec![6, 6, 10, 4, 2], 1)
    }

    #[test]
    fn rate_two_increases() {
        test_rate(&vec![6, 6, 10, 4, 2, 8, 4, 1], 2)
    }

    #[test]
    fn rate_example() {
        test_rate(&vec![199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263], 7)
    }

    fn test_sliding_rate(depths: &Vec<Depth>, expected: i32) {
        let mut problem = SlidingRateProblem{};
        let result = problem.solve(&depths).unwrap();
        assert_eq!(result, expected); 
    }

    #[test]
    fn sliding_rate_zero_inputs() {
        test_sliding_rate(&vec![], 0)
    }

    #[test]
    fn sliding_rate_one_input() {
        test_sliding_rate(&vec![8], 0)
    }

    #[test]
    fn sliding_rate_two_inputs() {
        test_sliding_rate(&vec![8, 9], 0)
    }

    #[test]
    fn sliding_rate_three_inputs() {
        test_sliding_rate(&vec![8, 9, 10], 0)
    }

    #[test]
    fn sliding_rate_zero_increases() {
        test_sliding_rate(&vec![8, 6, 4, 1], 0)
    }

    #[test]
    fn sliding_rate_one_increase() {
        test_sliding_rate(&vec![6, 6, 10, 12, 2], 1)
    }

    #[test]
    fn sliding_rate_two_increases() {
        test_sliding_rate(&vec![6, 6, 10, 12, 2, 8, 4, 9], 2)
    }

    #[test]
    fn sliding_rate_example() {
        test_sliding_rate(&vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263], 5)
    }
}
