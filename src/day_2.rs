enum Sign {
    Positive,
    Negative,
}
pub fn solve1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|lvl| lvl.parse::<i64>().expect("levels are ints"))
                .collect::<Vec<_>>()
        })
        .filter(|levels| validate_levels(levels))
        .count() as i64
}

pub fn solve2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|lvl| lvl.parse::<i64>().expect("levels are ints"))
                .collect::<Vec<_>>()
        })
        .filter(|levels| {
            let dampened_levels = (0..levels.len())
                .map(|dmpd_idx| {
                    levels
                        .iter()
                        .enumerate()
                        .filter_map(|(i, &lvl)| if i != dmpd_idx { Some(lvl) } else { None })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            return validate_levels(levels)
                || dampened_levels.iter().any(|levels| validate_levels(levels));
        })
        .count() as i64
}

fn validate_levels(levels: &[i64]) -> bool {
    let mut sign: Option<Sign> = None;
    for diff in levels.iter().map_windows(|&[lhs, rhs]| lhs - rhs) {
        match (&mut sign, diff.is_positive(), diff == 0 || diff.abs() > 3) {
            (_, _, true) | (Some(Sign::Positive), false, _) | (Some(Sign::Negative), true, _) => {
                return false;
            }
            (None, true, false) => sign = Some(Sign::Positive),
            (None, false, false) => sign = Some(Sign::Negative),
            _ => {}
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day_2::{solve1, solve2};

    #[test]
    fn solve1_test() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(solve1(&input), 2);
    }

    #[test]
    fn solve1_real() {
        let input = fs::read_to_string("inputs/d2p1").expect("input file is present");
        assert_eq!(solve1(&input), 242);
    }

    #[test]
    fn solve2_test() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(solve2(&input), 4);
    }

    #[test]
    fn solve2_real() {
        let input = fs::read_to_string("inputs/d2p1").expect("input file is present");
        assert_eq!(solve2(&input), 311);
    }
}
