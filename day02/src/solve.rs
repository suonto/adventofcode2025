type Id = u64;
type Range = (Id, Id);

fn parse_ranges(input: &str) -> Vec<Range> {
    input
        .split(",")
        .map(|range| {
            println!("Parsing range: {}", range.trim());
            let parts: Vec<&str> = range.trim().split('-').collect();
            let start: Id = parts[0].parse().unwrap();
            let end: Id = parts[1].parse().unwrap();
            (start, end)
        })
        .collect()
}

fn is_valid_id(id: Id) -> bool {
    let char_count = format!("{}", id).chars().count();
    if char_count % 2 != 0 {
        return true;
    }

    let half_count = char_count / 2;

    let as_string = format!("{}", id);
    let (first_half, second_half) = &as_string.split_at(half_count);

    let is_invalid = first_half == second_half;
    if is_invalid {
        println!("ID {} is invalid ({} == {})", id, first_half, second_half);
    }

    !is_invalid
}

fn count_invalid_ids(range: Range) -> Id {
    let (start, end) = range;
    let mut count: u64 = 0;
    for id in start..=end {
        if !is_valid_id(id) {
            count += id;
        }
    }
    count
}

pub fn solve_a(input: &str) -> Id {
    let mut result: u64 = 0;
    for range in parse_ranges(input) {
        println!("Parsed range: {}-{}", range.0, range.1);
        let invalid_count = count_invalid_ids(range);
        println!(
            "Invalid IDs in range {}-{}: {}",
            range.0, range.1, invalid_count
        );
        result += invalid_count;
    }

    println!("Total invalid IDs: {}", result);
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
        assert_eq!(solve_a(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(EXAMPLE), 0);
    }
}
