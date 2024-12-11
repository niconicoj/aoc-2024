use std::collections::HashMap;

pub fn solve(input: &str, blink_count: usize) -> usize {
    let mut starting_stones: HashMap<usize, usize> = HashMap::new();
    input
        .split_whitespace()
        .filter_map(|p| p.parse::<usize>().ok())
        .for_each(|n| match starting_stones.get_mut(&n) {
            Some(stone_qty) => *stone_qty += 1,
            None => {
                starting_stones.insert(n, 1);
            }
        });

    let final_stones = (0..blink_count).fold(starting_stones, |acc, _| {
        let mut next_stones = HashMap::new();
        acc.iter().for_each(|(n, &qty)| {
            match n {
                0 => add_or_update(1, qty, &mut next_stones),
                n if (n.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
                    let digits = n.checked_ilog10().unwrap_or(0) + 1;
                    let a = n / 10usize.pow(digits / 2);
                    let b = n - (a * 10usize.pow(digits / 2));
                    add_or_update(a, qty, &mut next_stones);
                    add_or_update(b, qty, &mut next_stones);
                }
                n => add_or_update(n * 2024, qty, &mut next_stones),
            };
        });

        next_stones
    });

    final_stones.into_values().sum()
}

fn add_or_update(key: usize, value: usize, map: &mut HashMap<usize, usize>) {
    match map.get_mut(&key) {
        Some(v) => *v += value,
        None => {
            map.insert(key, value);
        }
    }
}

pub fn solve2(input: &str) -> usize {
    let stones: HashMap<usize, usize> = HashMap::new();

    stones.len()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::solve;
    const INPUT_PATH: &str = "inputs/d11p1";
    const TEST_INPUT: &str = r#"125 17"#;

    #[test]
    fn solve1_test() {
        assert_eq!(solve(TEST_INPUT, 25), 55312);
    }

    #[test]
    fn solve1_real() {
        let input = fs::read_to_string(INPUT_PATH).expect("input file is present");
        assert_eq!(solve(&input, 25), 185894);
    }

    #[test]
    fn solve2_real() {
        let input = fs::read_to_string(INPUT_PATH).expect("input file is present");
        assert_eq!(solve(&input, 75), 221632504974231);
    }
}
