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

fn is_valid_id_a(id: Id) -> bool {
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

fn is_valid_id_b(id: Id) -> bool {
    let as_string = format!("{}", id);
    let char_count = &as_string.chars().count();

    for part_size in 1..((char_count / 2) + 1) {
        if part_size > 1 && char_count % part_size != 0 {
            continue;
        }

        let mut all_equal = true;

        let first_part = &as_string[0..part_size];

        for i in (part_size..as_string.len()).step_by(part_size) {
            let part = &as_string[i..i + part_size];
            if part != first_part {
                all_equal = false;
                break;
            }
        }

        if all_equal {
            println!("ID {} is invalid (repeated part {})", id, first_part);
            return false;
        }
    }

    return true;
}

fn count_invalid_ids(range: Range, variant_b: bool) -> Id {
    let (start, end) = range;
    let mut count: u64 = 0;
    for id in start..=end {
        if !variant_b && !is_valid_id_a(id) {
            count += id;
        }
        if variant_b && !is_valid_id_b(id) {
            count += id;
        }
    }
    count
}

pub fn solve_a(input: &str) -> Id {
    let mut result: u64 = 0;
    for range in parse_ranges(input) {
        println!("Parsed range: {}-{}", range.0, range.1);
        let invalid_count = count_invalid_ids(range, false);
        println!(
            "Invalid IDs in range {}-{}: {}",
            range.0, range.1, invalid_count
        );
        result += invalid_count;
    }

    println!("Total invalid IDs: {}", result);
    result
}

pub fn solve_b(input: &str) -> Id {
    let mut result: Id = 0;
    for range in parse_ranges(input) {
        let invalid_count = count_invalid_ids(range, true);
        println!(
            "Invalid IDs in range {}-{}: {}",
            range.0, range.1, invalid_count
        );
        result += invalid_count;
    }

    println!("Total invalid IDs: {}", result);
    result
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
        assert_eq!(solve_b(EXAMPLE), 4174379265);
    }

    #[test]
    fn test_11_22_b() {
        assert_eq!(count_invalid_ids((11, 22), true), 11 + 22);
    }

    #[test]
    fn test_998_1012_b() {
        assert_eq!(count_invalid_ids((998, 1012), true), 999 + 1010);
    }
}
