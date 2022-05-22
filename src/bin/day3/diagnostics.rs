use std::str::FromStr;

pub struct Diagnostics {
    epsilon: i32,
    gamma: i32,
    oxygen: i32,
    co2: i32
}

impl Diagnostics {
    pub fn power_consumption(&self) -> i32 {
        self.epsilon * self.gamma
    }

    pub fn life_support_rating(&self) -> i32 {
        self.oxygen * self.co2
    }

    pub fn from_report<T: AsRef<str>>(report: &[T]) -> Self {
        let (epsilon, gamma) = epsilon_and_gamma(report);
        let oxygen = from_criteria::<OxygenBitCriteria, T>(report);
        let co2 = from_criteria::<Co2BitCriteria, T>(report);

        Diagnostics { epsilon, gamma, oxygen, co2 }
    }
}

trait BitCriteria {
    fn new(bits: &[char]) -> Self;
    fn accept(&self, bit: &char) -> bool;
}

struct OxygenBitCriteria {
    desired: char
}

impl BitCriteria for OxygenBitCriteria {
    fn new(bits: &[char]) -> Self {
        let tally = tally(&bits);
        let desired = if tally.zeros > tally.ones {'0'} else {'1'};
        OxygenBitCriteria { desired }
    }

    fn accept(&self, bit: &char) -> bool {
        bit.eq(&self.desired)
    }
}

struct Co2BitCriteria {
    desired: char
}

impl BitCriteria for Co2BitCriteria {
    fn new(bits: &[char]) -> Self {
        let tally = tally(&bits);
        let desired = if tally.zeros > tally.ones {'1'} else {'0'};
        Co2BitCriteria { desired }
    }

    fn accept(&self, bit: &char) -> bool {
        bit.eq(&self.desired)
    }
}

struct Tally {
    zeros: i32,
    ones: i32
}

fn tally(bits: &[char]) -> Tally {
    let mut tally = Tally{ zeros: 0, ones: 0 };
    for c in bits {
        match c {
            '0' => tally.zeros += 1,
            '1' => tally.ones += 1,
            _ => panic!("Expected 1 or 0")
        }
    }

    tally
}

fn from_criteria<CriteriaT, StrT>(report: &[StrT]) -> i32 where CriteriaT : BitCriteria, StrT: AsRef<str> {
    let mut i = 0;
    let mut lines: Vec<String> = report.iter().map(|line| String::from_str(line.as_ref()).unwrap()).collect();

    while lines.len() > 1 {
        let bits: Vec<char> = lines.iter().map(|line| line.chars().nth(i).unwrap()).collect();
        let criteria = CriteriaT::new(&bits);
        lines.retain(|line| criteria.accept(&line.chars().nth(i).unwrap()));
        i += 1;
    }

    i32::from_str_radix(lines.first().unwrap(), 2).unwrap()
}

fn epsilon_and_gamma<T: AsRef<str>>(report: &[T]) -> (i32, i32) {
    let line_length = report.first().unwrap().as_ref().len();
    let mut counts = vec![(0, 0); line_length];

    for line in report {
        for (i, c) in line.as_ref().char_indices() {
            let mut tally = &mut counts[i];

            match c {
                '0' => tally.0 += 1,
                '1' => tally.1 += 1,
                _ => panic!("Bit is not binary")
            };
        }
    }

    let mut gamma_str: String = String::new();
    let mut epsilon_str: String = String::new();

    for (zeros, ones) in counts.iter() {
        if zeros > ones {
            gamma_str += "0";
            epsilon_str += "1";
        } else {
            gamma_str += "1";
            epsilon_str += "0";
        }
    }

    let gamma = i32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon_str, 2).unwrap();
    (epsilon, gamma)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> { vec![
    "00100".to_string(),
    "11110".to_string(),
    "10110".to_string(),
    "10111".to_string(),
    "10101".to_string(),
    "01111".to_string(),
    "00111".to_string(),
    "11100".to_string(),
    "10000".to_string(),
    "11001".to_string(),
    "00010".to_string(),
    "01010".to_string()]}

    #[test]
    fn test_power_consumption_example() {
        let diagnostics = Diagnostics::from_report(&example_input());
        assert_eq!(diagnostics.power_consumption(), 198)
    }

    #[test]
    fn test_life_support_example() {
        let diagnostics = Diagnostics::from_report(&example_input());
        assert_eq!(diagnostics.life_support_rating(), 230)
    }
}