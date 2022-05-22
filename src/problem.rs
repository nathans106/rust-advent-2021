
use std::fmt;

#[derive(Debug)]
pub struct ProblemError {}

impl fmt::Display for ProblemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Oopsy daisy")
    }
}

pub trait Problem<InputT> {
    fn solve(&mut self, input: &InputT) -> Result<i32, ProblemError>;
}
