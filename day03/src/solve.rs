type Joltage = u64;
type Bank = Vec<Joltage>;

fn parse_banks(input: &str) -> Vec<Bank> {
    let mut banks: Vec<Bank> = Vec::new();
    for raw_bank in input.lines() {
        let bank: Bank = raw_bank
            .trim()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as Joltage)
            .collect();

        if bank.len() == 0 {
            continue;
        }
        banks.push(bank);
    }
    banks
}

fn get_bank_max_after(bank: &Bank, after: Option<usize>, max_index: usize) -> (usize, Joltage) {
    let mut max_val = 0;
    let mut max_val_index = 0;
    let start = after.map(|a| a.saturating_add(1)).unwrap_or(0);
    for (i, &joltage) in bank.iter().enumerate().skip(start) {
        // do not accept greater than max_index
        if i > max_index {
            break;
        }

        if joltage > max_val {
            max_val = joltage;
            max_val_index = i;
            if max_val == 9 {
                break;
            }
        }
    }
    (max_val_index, max_val)
}

fn get_bank_joltage_a(bank: &Bank) -> Joltage {
    let (first_index, first) = get_bank_max_after(&bank, None, bank.len() - 2);
    let (_, second) = get_bank_max_after(&bank, Some(first_index), bank.len().saturating_sub(1));

    first * 10 + second
}

fn get_bank_joltage_b(bank: &Bank) -> Joltage {
    let mut joltages: Vec<Joltage> = Vec::new();
    let mut after: Option<usize> = None;
    for i in 0..12 {
        let (index, joltage) = get_bank_max_after(&bank, after, bank.len() - 12 + i);
        after = Some(index);
        joltages.push(joltage);
    }

    joltages
        .into_iter()
        .reduce(|acc, curr| acc * 10 + curr)
        .unwrap()
}

pub fn solve_a(input: &str) -> u64 {
    let mut result: u64 = 0;
    for bank in parse_banks(input) {
        let joltage = get_bank_joltage_a(&bank);
        println!("bank {:?} max joltage: {}", bank, joltage);
        result += joltage as u64;
    }

    println!("Total joltage: {}", result);
    result
}

pub fn solve_b(input: &str) -> u64 {
    let mut result: u64 = 0;
    for bank in parse_banks(input) {
        let joltage = get_bank_joltage_b(&bank);
        println!("bank {:?} max joltage: {}", bank, joltage);
        result += joltage as u64;
    }

    println!("Total joltage: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a(EXAMPLE), 357);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(EXAMPLE), 3121910778619);
    }
}
