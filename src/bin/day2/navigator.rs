pub enum Direction {
    Forward,
    Up,
    Down
}

pub type Instructions = Vec<Instruction>;

pub struct Instruction {
    pub direction: Direction,
    pub distance: i32
}

pub struct Position {
    pub depth: i32,
    pub horizontal: i32
}

pub trait Navigator {
    fn calculate_position(instructions: &[Instruction]) -> Position;
}

pub struct LinearNavigator {}
impl Navigator for LinearNavigator {
    fn calculate_position(instructions: &[Instruction]) -> Position {
        let mut pos = Position{depth: 0, horizontal: 0 };

        for instruction in instructions {
            match instruction.direction {
                Direction::Forward => pos.horizontal += instruction.distance,
                Direction::Up => pos.depth -= instruction.distance,
                Direction::Down => pos.depth += instruction.distance,
            }
        }
    
        pos
    }
}

pub struct AngularNavigator {}
impl Navigator for AngularNavigator {
    fn calculate_position(instructions: &[Instruction]) -> Position {
        let mut pos = Position{depth: 0, horizontal: 0 };
        let mut aim = 0;


        for instruction in instructions {
            match instruction.direction {
                Direction::Forward => {
                    pos.horizontal += instruction.distance;
                    pos.depth += aim * instruction.distance
                },
                Direction::Up => aim -= instruction.distance,
                Direction::Down => aim += instruction.distance,
            }
        }
    
        pos
    }
}
