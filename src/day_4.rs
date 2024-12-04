pub fn solve1(input: &str) -> i64 {
    let mut count = 0;
    let input = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for y in 0..input.len() {
        let row = input.get(y).expect("row is defined");
        for x in 0..row.len() {
            count += [
                (
                    (x, y),
                    (x, y.wrapping_sub(1)),
                    (x, y.wrapping_sub(2)),
                    (x, y.wrapping_sub(3)),
                ),
                (
                    (x, y),
                    (x + 1, y.wrapping_sub(1)),
                    (x + 2, y.wrapping_sub(2)),
                    (x + 3, y.wrapping_sub(3)),
                ),
                ((x, y), (x + 1, y), (x + 2, y), (x + 3, y)),
                ((x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)),
                ((x, y), (x, y + 1), (x, y + 2), (x, y + 3)),
                (
                    (x, y),
                    (x.wrapping_sub(1), y + 1),
                    (x.wrapping_sub(2), y + 2),
                    (x.wrapping_sub(3), y + 3),
                ),
                (
                    (x, y),
                    (x.wrapping_sub(1), y),
                    (x.wrapping_sub(2), y),
                    (x.wrapping_sub(3), y),
                ),
                (
                    (x, y),
                    (x.wrapping_sub(1), y.wrapping_sub(1)),
                    (x.wrapping_sub(2), y.wrapping_sub(2)),
                    (x.wrapping_sub(3), y.wrapping_sub(3)),
                ),
            ]
            .iter()
            .filter(|&p| {
                match (
                    input.get(p.0 .1).and_then(|row| row.get(p.0 .0)),
                    input.get(p.1 .1).and_then(|row| row.get(p.1 .0)),
                    input.get(p.2 .1).and_then(|row| row.get(p.2 .0)),
                    input.get(p.3 .1).and_then(|row| row.get(p.3 .0)),
                ) {
                    (Some('X'), Some('M'), Some('A'), Some('S')) => true,
                    _ => false,
                }
            })
            .count() as i64
        }
    }
    count
}

pub fn solve2(input: &str) -> i64 {
    let mut count = 0;
    let input = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for y in 0..input.len() {
        let row = input.get(y).expect("row is defined");
        for x in 0..row.len() {
            match (
                input.get(y).and_then(|row| row.get(x)),
                input.get(y.wrapping_sub(1)).and_then(|row| row.get(x + 1)),
                input.get(y + 1).and_then(|row| row.get(x + 1)),
                input.get(y + 1).and_then(|row| row.get(x.wrapping_sub(1))),
                input
                    .get(y.wrapping_sub(1))
                    .and_then(|row| row.get(x.wrapping_sub(1))),
            ) {
                (Some('A'), Some('M'), Some('M'), Some('S'), Some('S'))
                | (Some('A'), Some('M'), Some('S'), Some('S'), Some('M'))
                | (Some('A'), Some('S'), Some('S'), Some('M'), Some('M'))
                | (Some('A'), Some('S'), Some('M'), Some('M'), Some('S')) => {
                    count += 1;
                }
                _ => {}
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{solve1, solve2};

    #[test]
    fn solve1_test() {
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(solve1(&input), 18);
    }

    #[test]
    fn solve1_real() {
        let input = fs::read_to_string("inputs/d4p1").expect("input file is present");
        assert_eq!(solve1(&input), 2599);
    }

    #[test]
    fn solve2_test() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(solve2(&input), 9);
    }

    #[test]
    fn solve2_real() {
        let input = fs::read_to_string("inputs/d4p1").expect("input file is present");
        assert_eq!(solve2(&input), 1948);
    }
}
