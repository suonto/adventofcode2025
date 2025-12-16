use std::collections::HashMap;

use direction::{Coord, DirectionIter};

use crate::paper_roll::PaperRoll;

type Grid = HashMap<Coord, PaperRoll>;

fn parse_grid(input: &str) -> Grid {
    let mut grid: HashMap<Coord, PaperRoll> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '@' => {
                    let roll = PaperRoll::new(x as i32, y as i32);
                    grid.insert(roll.pos, roll);
                }
                '.' => {}
                _ => panic!("Invalid character in grid: {}", ch),
            };
        }
    }

    grid
}

pub fn neighbor_positions(roll: &PaperRoll) -> Vec<Coord> {
    DirectionIter::new()
        .map(|dir| roll.pos + dir.coord())
        .collect()
}

fn accessible_by_forklift(grid: &Grid) -> Vec<Coord> {
    let mut result: Vec<Coord> = Vec::new();
    for roll in grid.values() {
        let neighbor_positions = neighbor_positions(roll);
        let mut neighbor_count = 0;
        for pos in neighbor_positions {
            if grid.contains_key(&pos) {
                neighbor_count += 1;
            }
        }
        if neighbor_count < 4 {
            result.push(roll.pos);
        }
    }

    println!("Forklift accessible rolls: {}", result.len());
    result
}

pub fn solve_a(input: &str) -> u64 {
    let grid = parse_grid(input);

    accessible_by_forklift(&grid).len() as u64
}

pub fn solve_b(input: &str) -> u64 {
    let mut grid = parse_grid(input);
    let mut result: u64 = 0;

    let mut accessible_roll_positions = accessible_by_forklift(&grid);
    while accessible_roll_positions.len() > 0 {
        let mut iteration_result = 0;
        for roll_pos in accessible_roll_positions {
            grid.remove(&roll_pos);
            iteration_result += 1;
        }
        println!("Remove {} rolls of paper", iteration_result);
        result += iteration_result;
        accessible_roll_positions = accessible_by_forklift(&grid);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a(EXAMPLE), 13);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(EXAMPLE), 43);
    }
}
