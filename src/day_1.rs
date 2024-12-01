use std::collections::HashMap;

pub fn solve1(input: &str) -> i64 {
    let (mut lhs, mut rhs): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            let (l, r) = l
                .split_once(' ')
                .expect("each line should contains at least one space");
            (
                l.trim().parse::<i64>().expect("lhs should be an int"),
                r.trim().parse::<i64>().expect("rhs should be an int"),
            )
        })
        .unzip();

    lhs.sort();
    rhs.sort();

    lhs.iter()
        .zip(rhs.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs())
}

pub fn solve2(input: &str) -> i64 {
    let (lhs, rhs): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            let (l, r) = l
                .split_once(' ')
                .expect("each line should contains at least one space");
            (
                l.trim().parse::<i64>().expect("lhs should be an int"),
                r.trim().parse::<i64>().expect("rhs should be an int"),
            )
        })
        .unzip();

    let mut rhs_map: HashMap<i64, i64> = HashMap::new();
    rhs.iter().for_each(|el| match rhs_map.get_mut(el) {
        Some(count) => *count += 1,
        None => {
            rhs_map.insert(*el, 1);
        }
    });

    lhs.iter()
        .fold(0, |acc, el| acc + el * rhs_map.get(el).unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day_1::{solve1, solve2};

    #[test]
    fn solve1_test() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(solve1(&input), 11);
    }

    #[test]
    fn solve1_real() {
        let input = fs::read_to_string("inputs/d1p1").expect("input file is present");
        assert_eq!(solve1(&input), 3569916);
    }

    #[test]
    fn solve2_test() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(solve2(&input), 31);
    }

    #[test]
    fn solve2_real() {
        let input = fs::read_to_string("inputs/d1p1").expect("input file is present");
        assert_eq!(solve2(&input), 26407426);
    }
}
