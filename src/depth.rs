
pub type Depth = i16;

pub fn rate(depths: &[Depth]) -> i32 {
    depths.windows(2).filter(|&pair| pair[1] > pair[0]).count().try_into().unwrap()
}

pub fn sliding_rate(depths: &[Depth]) -> i32 {
    let windowed: Vec<i16> = depths.windows(3).map(|window| window.iter().sum()).collect();
    rate(&windowed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rate(depths: &[Depth], expected: i32) {
        let result = rate(depths);
        assert_eq!(result, expected); 
    }

    #[test]
    fn rate_zero_inputs() {
        test_rate(&[], 0)
    }

    #[test]
    fn rate_one_input() {
        test_rate(&[8], 0)
    }

    #[test]
    fn rate_zero_increases() {
        test_rate(&[8, 6, 4], 0)
    }

    #[test]
    fn rate_one_increase() {
        test_rate(&[6, 6, 10, 4, 2], 1)
    }

    #[test]
    fn rate_two_increases() {
        test_rate(&[6, 6, 10, 4, 2, 8, 4, 1], 2)
    }

    #[test]
    fn rate_example() {
        test_rate(&[199,
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

    fn test_sliding_rate(depths: &[Depth], expected: i32) {
        let result = sliding_rate(depths);
        assert_eq!(result, expected); 
    }

    #[test]
    fn sliding_rate_zero_inputs() {
        test_sliding_rate(&[], 0)
    }

    #[test]
    fn sliding_rate_one_input() {
        test_sliding_rate(&[8], 0)
    }

    #[test]
    fn sliding_rate_two_inputs() {
        test_sliding_rate(&[8, 9], 0)
    }

    #[test]
    fn sliding_rate_three_inputs() {
        test_sliding_rate(&[8, 9, 10], 0)
    }

    #[test]
    fn sliding_rate_zero_increases() {
        test_sliding_rate(&[8, 6, 4, 1], 0)
    }

    #[test]
    fn sliding_rate_one_increase() {
        test_sliding_rate(&[6, 6, 10, 12, 2], 1)
    }

    #[test]
    fn sliding_rate_two_increases() {
        test_sliding_rate(&[6, 6, 10, 12, 2, 8, 4, 9], 2)
    }

    #[test]
    fn sliding_rate_example() {
        test_sliding_rate(&[
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
