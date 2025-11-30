use direction::CardinalDirection;
use jagger::Jagger;

pub fn solve_a(input: &str) -> String {
    let mut j = Jagger::new(0, 0);
    for line in input.lines() {
        let ch = line.trim().chars().next();
        if let Some(dir) = ch {
            match dir {
                'N' => j.mv(CardinalDirection::North),
                'E' => j.mv(CardinalDirection::East),
                'S' => j.mv(CardinalDirection::South),
                'W' => j.mv(CardinalDirection::West),
                _ => {}
            }
        }
    }
    format!("{},{}", j.pos.x, j.pos.y)
}

pub fn solve_b(input: &str) -> String {
    // For part B we perform the same moves but start at (10,10)
    let mut j = Jagger::new(10, 10);
    for line in input.lines() {
        let ch = line.trim().chars().next();
        if let Some(dir) = ch {
            match dir {
                'N' => j.mv(CardinalDirection::North),
                'E' => j.mv(CardinalDirection::East),
                'S' => j.mv(CardinalDirection::South),
                'W' => j.mv(CardinalDirection::West),
                _ => {}
            }
        }
    }
    format!("{},{}", j.pos.x, j.pos.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::*;

    #[test]
    fn test_a() {
        // EXAMPLE_A contains two 'S' moves starting from (0,0) -> (0,2)
        let expected = "0,2";
        assert_eq!(solve_a(EXAMPLE_A), expected);
    }

    #[test]
    fn test_b() {
        // EXAMPLE_B contains two 'N' moves starting from (10,10) -> (10,8)
        let expected = "10,8";
        assert_eq!(solve_b(EXAMPLE_B), expected);
    }
}
