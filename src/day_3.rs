pub fn solve1(input: &str) -> i64 {
    let mut pointer: usize = 0;
    let mut counter = 0;

    while let Some(candidate) = input.get(pointer..pointer + 4) {
        if candidate == "mul(" {
            pointer += 4;
            if let Some((lhs, lhs_len)) = parse_int(input, pointer) {
                pointer += lhs_len;
                if input.get(pointer..pointer + 1) == Some(",") {
                    pointer += 1;
                    if let Some((rhs, rhs_len)) = parse_int(input, pointer) {
                        pointer += rhs_len;
                        if input.get(pointer..pointer + 1) == Some(")") {
                            counter += lhs * rhs;
                        }
                    }
                }
            } else {
            }
        } else {
            pointer += 1;
        }
    }

    counter
}

pub fn parse_int(input: &str, pointer: usize) -> Option<(i64, usize)> {
    let mut int_len = 0;
    let mut int = None;
    while let Some(i) = input
        .get(pointer..pointer + int_len + 1)
        .map(|n| n.parse::<i64>().ok())
        .flatten()
    {
        int_len += 1;
        int = Some(i);
    }
    int.map(|i| (i, int_len))
}

pub fn solve2(input: &str) -> i64 {
    let mut pointer: usize = 0;
    let mut counter = 0;
    let mut enabled = true;

    while let Some(candidate) = input.get(pointer..pointer + 7) {
        if enabled {
            if candidate.starts_with("mul(") {
                pointer += 4;
                if let Some((lhs, lhs_len)) = parse_int(input, pointer) {
                    pointer += lhs_len;
                    if input.get(pointer..pointer + 1) == Some(",") {
                        pointer += 1;
                        if let Some((rhs, rhs_len)) = parse_int(input, pointer) {
                            pointer += rhs_len;
                            if input.get(pointer..pointer + 1) == Some(")") {
                                counter += lhs * rhs;
                            }
                        }
                    }
                }
            } else if candidate == "don't()" {
                enabled = false;
                pointer += 7;
            } else {
                pointer += 1;
            }
        } else {
            if candidate.starts_with("do()") {
                enabled = true;
                pointer += 4;
            } else {
                pointer += 1;
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{solve1, solve2};

    #[test]
    fn solve1_test() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(solve1(&input), 161);
    }

    #[test]
    fn solve1_real() {
        let input = fs::read_to_string("inputs/d3p1").expect("input file is present");
        assert_eq!(solve1(&input), 170778545);
    }

    #[test]
    fn solve2_test() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(solve2(&input), 48);
    }

    #[test]
    fn solve2_real() {
        let input = fs::read_to_string("inputs/d3p1").expect("input file is present");
        assert_eq!(solve2(&input), 82868252);
    }
}
