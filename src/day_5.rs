use std::collections::HashMap;

struct RuleSet {
    rules: HashMap<i64, Vec<i64>>,
}

impl RuleSet {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }
    fn add_rule(&mut self, page: i64, rule: i64) {
        if let Some(rules) = self.rules.get_mut(&rule) {
            rules.push(page);
        } else {
            self.rules.insert(rule, vec![page]);
        }
    }

    fn check(&self, page: i64, pending: &[i64]) -> Option<usize> {
        self.rules
            .get(&page)
            .map(|rules| {
                rules
                    .iter()
                    .find_map(|rule| pending.iter().position(|p| p == rule))
            })
            .flatten()
    }
}

pub fn solve1(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut ruleset = RuleSet::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        } else {
            let (lhs, rhs) = line
                .split_once('|')
                .expect("rule line should contain a pipe");
            let lhs = lhs.parse::<i64>().expect("lhs should be an int");
            let rhs = rhs.parse::<i64>().expect("rhs should be an int");
            ruleset.add_rule(lhs, rhs);
        }
    }

    let mut result = 0;

    while let Some(line) = lines.next() {
        let printing_update = line
            .split(',')
            .map(|p| {
                p.parse::<i64>()
                    .expect("update should be comma separated ints")
            })
            .collect::<Vec<_>>();

        println!("=== checking update {:?} ===", printing_update);
        if printing_update.iter().enumerate().all(|(idx, &page)| {
            let result = ruleset
                .check(
                    page,
                    printing_update
                        .get(idx..printing_update.len())
                        .expect("check range should be correct"),
                )
                .is_none();
            println!("page {} check result : {}", idx, result);
            result
        }) {
            println!("printing update {:?} is correctly ordered", printing_update);
            result += printing_update
                .get(printing_update.len() / 2)
                .expect("update should have a middle");
        };
    }
    result
}

pub fn solve2(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut ruleset = RuleSet::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        } else {
            let (lhs, rhs) = line
                .split_once('|')
                .expect("rule line should contain a pipe");
            let lhs = lhs.parse::<i64>().expect("lhs should be an int");
            let rhs = rhs.parse::<i64>().expect("rhs should be an int");
            ruleset.add_rule(lhs, rhs);
        }
    }

    let mut lines = lines
        .map(|line| {
            line.split(',')
                .map(|p| {
                    p.parse::<i64>()
                        .expect("update should be comma separated ints")
                })
                .collect::<Vec<_>>()
        })
        .filter(|line| {
            line.iter().enumerate().any(|(idx, &page)| {
                ruleset
                    .check(
                        page,
                        line.get(idx..line.len())
                            .expect("check range should be correct"),
                    )
                    .is_some()
            })
        });

    let mut result = 0;
    while let Some(mut line) = lines.next() {
        let mut limiter = 10_000;
        while let Some(err) = line.iter().enumerate().find_map(|(idx, &page)| {
            ruleset
                .check(
                    page,
                    line.get(idx..line.len())
                        .expect("check range should be correct"),
                )
                .map(|r| (idx, r + idx))
        }) {
            println!("swapping {} and {}", err.0, err.1);
            line.swap(err.0, err.1);
            limiter -= 1;
            if limiter == 0 {
                break;
            }
        }
        if limiter != 0 {
            result += line.get(line.len() / 2).expect("line should have a middle");
        } else {
            panic!("reached max iterations");
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{RuleSet, solve1, solve2};
    const INPUT_PATH: &str = "inputs/d5p1";

    #[test]
    fn solve1_test() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(solve1(&input), 143);
    }

    #[test]
    fn solve1_real() {
        let input = fs::read_to_string(INPUT_PATH).expect("input file is present");
        assert_eq!(solve1(&input), 5991);
    }

    #[test]
    fn solve2_test() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(solve2(&input), 123);
    }

    #[test]
    fn solve2_real() {
        let input = fs::read_to_string(INPUT_PATH).expect("input file is present");
        assert_eq!(solve2(&input), 5479);
    }

    #[test]
    fn ruleset_test() {
        let mut ruleset = RuleSet::new();

        assert_eq!(ruleset.check(10, &[12, 13, 14]), None);
        ruleset.add_rule(13, 10);
        assert_eq!(ruleset.check(10, &[12, 13, 14]), Some(1));
    }
}
