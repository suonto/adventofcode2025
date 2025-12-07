use crate::dial::Dial;

pub fn solve_a(input: &str) -> u32 {
    let mut result: u32 = 0;
    let mut dial = Dial::from(50);
    for instruction in input.lines() {
        dial.rotate(instruction, false);
        if dial.pos == 0 {
            result += 1;
        }
    }

    result
}

pub fn solve_b(input: &str) -> u32 {
    let mut result: u32 = 0;
    let mut dial = Dial::from(50);
    for instruction in input.lines() {
        result += dial.rotate(instruction, true);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a(EXAMPLE), 3);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(EXAMPLE), 6);
    }
}
