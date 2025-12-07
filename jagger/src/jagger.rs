use direction::{CardinalDirection, Coord};

// Jagger can move and greet from a location.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Jagger {
    pub pos: Coord,
}

impl Jagger {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos: Coord::new(x, y),
        }
    }

    pub fn mv(&mut self, dir: CardinalDirection) {
        self.pos += dir.coord();
    }

    pub fn greet_from_location(&self) {
        println!("Greetings from pos ({}, {})!", self.pos.x, self.pos.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_mv() {
        let mut jagger = Jagger::new(0, 0);
        jagger.mv(CardinalDirection::North);
        assert_eq!(jagger.pos, Coord::new(0, -1));
        jagger.mv(CardinalDirection::East);
        assert_eq!(jagger.pos, Coord::new(1, -1));
        jagger.mv(CardinalDirection::South);
        assert_eq!(jagger.pos, Coord::new(1, 0));
        jagger.mv(CardinalDirection::West);
        assert_eq!(jagger.pos, Coord::new(0, 0));
    }
}
