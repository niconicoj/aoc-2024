use std::collections::HashSet;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn advance(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (position.0, position.1.wrapping_sub(1)),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0.wrapping_sub(1), position.1),
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn solve1(input: &str) -> i64 {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut position = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if c == &'^' { Some((x, y)) } else { None })
        })
        .expect("starting position exists");

    let mut direction = Direction::Up;
    let mut visited = HashSet::new();
    visited.insert(position);

    while let Some((next_position, next_direction)) = get_next_position(direction, position, &map) {
        position = next_position;
        direction = next_direction;
        visited.insert(position);
    }

    visited.len() as i64
}

pub fn solve2(input: &str) -> i64 {
    todo!()
}

fn get_next_position(
    direction: Direction,
    position: (usize, usize),
    map: &Vec<Vec<char>>,
) -> Option<((usize, usize), Direction)> {
    let next_position = direction.advance(position);
    match map
        .get(next_position.1)
        .and_then(|row| row.get(next_position.0))
    {
        Some('#') => get_next_position(direction.rotate(), position, map),
        Some(_) => Some((next_position, direction)),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{solve1, solve2};
    const INPUT_PATH: &str = "inputs/d6p1";

    #[test]
    fn solve1_test() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(solve1(&input), 41);
    }

    #[test]
    fn solve1_real() {
        let input = fs::read_to_string(INPUT_PATH).expect("input file is present");
        assert_eq!(solve1(&input), 4580);
    }

    // #[test]
    fn solve2_test() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(solve2(&input), 123);
    }

    // #[test]
    fn solve2_real() {
        let input = fs::read_to_string(INPUT_PATH).expect("input file is present");
        assert_eq!(solve2(&input), 0);
    }
}
