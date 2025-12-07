pub enum RotationDirection {
    L,
    R,
}

impl From<Option<char>> for RotationDirection {
    fn from(c: Option<char>) -> Self {
        match c {
            Some('L') => RotationDirection::L,
            Some('R') => RotationDirection::R,
            _ => panic!("Invalid rotation direction character: {:?}", c),
        }
    }
}
