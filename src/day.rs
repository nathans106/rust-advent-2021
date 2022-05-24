use std::time;

use crate::problem::Problem;

pub struct Day<InputT> {
    id: i8,
    problems: Vec<Box<dyn Problem<InputT>>>,
    input: InputT,
}

impl<InputT> Day<InputT> {
    pub fn new(id: i8, input: InputT) -> Self {
        Day {
            id: id,
            problems: vec![],
            input: input,
        }
    }

    pub fn add_problem<P: Problem<InputT> + 'static>(&mut self, problem: P) {
        self.problems.push(Box::new(problem));
    }

    pub fn solve(&mut self) {
        print!("Day {} solving {} problems\n", self.id, self.problems.len());

        for (i, problem) in self.problems.iter_mut().enumerate() {
            let id = i + 1;

            let start_time = time::Instant::now();
            let result = problem.solve(&self.input);
            let duration = start_time.elapsed();

            match result {
                Ok(value) => {
                    print!("Problem {} succesful! Result {}\n", id, value);
                }
                Err(e) => {
                    print!("Problem {} failed! Error: {}\n", id, e);
                }
            }

            print!("Duration: {} us\n", duration.as_micros());
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::problem::ProblemError;

    use super::*;

    struct MockProblem {}

    impl Problem<i8> for MockProblem {
        fn solve(&mut self, _input: &i8) -> Result<i32, ProblemError> {
            Ok(5)
        }
    }

    #[test]
    fn zero_problems() {
        let mut day = Day::<i8>::new(3, 9);
        day.solve();
    }

    #[test]
    fn one_problem() {
        let mut day = Day::<i8>::new(3, 9);
        let problem = MockProblem {};
        day.add_problem(problem);
        day.solve();
    }
}
