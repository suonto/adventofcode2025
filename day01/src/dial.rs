use crate::rotation_direction::RotationDirection;

pub type DialPos = u8;

// Dial points to a number between 0 and 99.
// Dial implements rotation to left or right by a given count.
pub struct Dial {
    pub pos: DialPos,
}

impl From<u8> for Dial {
    fn from(pos: u8) -> Self {
        match pos {
            0..=99 => {
                println!("The dial starts by pointing at {}", pos);
                Dial { pos }
            }
            _ => panic!("Invalid dial pos: {}", pos),
        }
    }
}

impl Dial {
    pub fn rotate(&mut self, instruction: &str, count_zero_ticks: bool) -> u32 {
        let direction_char = instruction.trim().chars().next();
        let direction = RotationDirection::from(direction_char);
        let count: u32 = instruction[1..].parse().unwrap();
        let result = self.rotate_dir_n(direction, count);
        let mut msg = format!(
            "The dial is rotated {} to point at {}",
            instruction, self.pos
        );
        if count_zero_ticks {
            msg += &format!("; during this rotation, it points at 0 {} time(s).", result)
        } else {
            msg += ".";
        }
        println!("{}", msg);
        result
    }

    // Rotate the dial n steps to direction.
    // Returns the count of times the dial pointed at 0 during the rotation.
    fn rotate_dir_n(&mut self, direction: RotationDirection, count: u32) -> u32 {
        let rounds = count / 100;
        let mut point_at_zero_times = rounds;
        let diff: i16 = match direction {
            RotationDirection::L => {
                let diff = -((count % 100) as i16);
                if self.pos > 0 && (self.pos as i16) + diff <= 0 {
                    point_at_zero_times += 1;
                }
                diff
            }
            RotationDirection::R => {
                let diff = (count % 100) as i16;
                if (self.pos as i16) + diff >= 100 {
                    point_at_zero_times += 1;
                }
                diff
            }
        };

        self.pos = ((self.pos as i16 + diff + 100) % 100) as DialPos;
        point_at_zero_times
    }
}
