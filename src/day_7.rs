pub fn solve1(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| line.split_once(':'))
        .filter_map(|(lhs, rhs)| {
            lhs.parse::<i64>()
                .map(|lhs| {
                    (
                        lhs,
                        rhs.split_whitespace()
                            .filter_map(|p| p.parse::<i64>().ok())
                            .collect::<Vec<_>>(),
                    )
                })
                .ok()
        })
        .fold(0, |acc, equation| match test_equation(&equation) {
            true => acc + equation.0,
            false => acc,
        })
}

fn test_equation(equation: &(i64, Vec<i64>)) -> bool {
    if let Some((lhs, rhs)) = equation.1.get(0).zip(equation.1.get(1)) {
        test_equation_recurse(equation, lhs + rhs, 1)
            || test_equation_recurse(equation, lhs * rhs, 1)
    } else {
        equation.0 == 0
    }
}

fn test_equation_recurse(equation: &(i64, Vec<i64>), acc: i64, index: usize) -> bool {
    if let Some(el) = equation.1.get(index + 1) {
        test_equation_recurse(equation, acc + el, index + 1)
            || test_equation_recurse(equation, acc * el, index + 1)
    } else {
        acc == equation.0
    }
}

pub fn solve2(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| line.split_once(':'))
        .filter_map(|(lhs, rhs)| {
            lhs.parse::<i64>()
                .map(|lhs| {
                    (
                        lhs,
                        rhs.split_whitespace()
                            .filter_map(|p| p.parse::<i64>().ok())
                            .collect::<Vec<_>>(),
                    )
                })
                .ok()
        })
        .fold(0, |acc, equation| match test_equation2(&equation) {
            true => acc + equation.0,
            false => acc,
        })
}

fn test_equation2(equation: &(i64, Vec<i64>)) -> bool {
    if let Some((lhs, rhs)) = equation.1.get(0).zip(equation.1.get(1)) {
        test_equation_recurse2(equation, lhs + rhs, 1)
            || test_equation_recurse2(equation, lhs * rhs, 1)
            || test_equation_recurse2(
                equation,
                lhs * 10i64.pow(rhs.checked_ilog10().unwrap_or(0) + 1) + rhs,
                1,
            )
    } else {
        equation.0 == 0
    }
}

fn test_equation_recurse2(equation: &(i64, Vec<i64>), acc: i64, index: usize) -> bool {
    if let Some(el) = equation.1.get(index + 1) {
        test_equation_recurse2(equation, acc + el, index + 1)
            || test_equation_recurse2(equation, acc * el, index + 1)
            || test_equation_recurse2(
                equation,
                acc * 10i64.pow(el.checked_ilog10().unwrap_or(0) + 1) + el,
                index + 1,
            )
    } else {
        acc == equation.0
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{solve1, solve2};
    const INPUT_PATH: &str = "inputs/d7p1";

    #[test]
    fn solve1_test() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!(solve1(&input), 3749);
    }

    #[test]
    fn solve1_real() {
        let input = fs::read_to_string(INPUT_PATH).expect("input file is present");
        assert_eq!(solve1(&input), 21572148763543);
    }

    #[test]
    fn solve2_test() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!(solve2(&input), 11387);
    }

    #[test]
    fn solve2_real() {
        let input = fs::read_to_string(INPUT_PATH).expect("input file is present");
        assert_eq!(solve2(&input), 581941094529163);
    }
}
