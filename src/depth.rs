
pub type Depth = i16;

pub fn rate(depths: &[Depth]) -> i32 {
    depths.windows(2).filter(|&pair| pair[1] > pair[0]).count().try_into().unwrap()
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
}