use crate::dial::Dial;

pub fn solve_a(input: &str) -> u32 {
    let mut result: u32 = 0;
    let mut dial = Dial::from(50);
    for instruction in input.lines() {
        dial.rotate(instruction);
        if dial.pos == 0 {
            result += 1;
        }
    }

    result
}

pub fn solve_b(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a(EXAMPLE_A), 3);
    }

    #[test]
    fn test_b() {
        // EXAMPLE_B contains two 'N' moves starting from (10,10) -> (10,8)
        assert_eq!(solve_b(EXAMPLE_B), 0);
    }
}
