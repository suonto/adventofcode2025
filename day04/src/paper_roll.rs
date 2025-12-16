use direction::Coord;

#[derive(Debug)]
pub struct PaperRoll {
    pub pos: Coord,
}

impl PaperRoll {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos: Coord::new(x, y),
        }
    }
}
