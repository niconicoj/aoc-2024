use std::collections::{HashMap, HashSet};

pub fn solve1(input: &str) -> usize {
    let (x, y) = (
        input
            .lines()
            .next()
            .expect("at least one line")
            .chars()
            .count() as isize,
        input.lines().count() as isize,
    );

    let mut map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, char)| char != &'.')
            .for_each(|(x, char)| {
                match map.get_mut(&char) {
                    Some(antennas) => antennas.push((x as isize, y as isize)),
                    None => {
                        map.insert(char, vec![(x as isize, y as isize)]);
                    }
                };
            })
    });

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    map.iter().for_each(|(_, antennas)| {
        (0..antennas.len() - 1).for_each(|idx| {
            let a = antennas.get(idx).expect("antenna a is defined");
            (idx + 1..antennas.len()).for_each(|idy| {
                let b = antennas.get(idy).expect("antenna b is defined");
                let v = (b.0 - a.0, b.1 - a.1);
                let (antinode_a, antinode_b) = ((b.0 + v.0, b.1 + v.1), (a.0 - v.0, a.1 - v.1));
                if antinode_a.0 >= 0 && antinode_a.0 < x && antinode_a.1 >= 0 && antinode_a.1 < y {
                    antinodes.insert(antinode_a);
                }
                if antinode_b.0 >= 0 && antinode_b.0 < x && antinode_b.1 >= 0 && antinode_b.1 < y {
                    antinodes.insert(antinode_b);
                }
            })
        });
    });
    antinodes.len()
}

pub fn solve2(input: &str) -> usize {
    let (x, y) = (
        input
            .lines()
            .next()
            .expect("at least one line")
            .chars()
            .count() as isize,
        input.lines().count() as isize,
    );

    let mut map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, char)| char != &'.')
            .for_each(|(x, char)| {
                match map.get_mut(&char) {
                    Some(antennas) => antennas.push((x as isize, y as isize)),
                    None => {
                        map.insert(char, vec![(x as isize, y as isize)]);
                    }
                };
            })
    });

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    map.iter().for_each(|(_, antennas)| {
        (0..antennas.len() - 1).for_each(|idx| {
            let a = antennas.get(idx).expect("antenna a is defined");
            antinodes.insert(a.clone());
            (idx + 1..antennas.len()).for_each(|idy| {
                let b = antennas.get(idy).expect("antenna b is defined");
                antinodes.insert(b.clone());
                let v = (b.0 - a.0, b.1 - a.1);
                let (mut antinode_a, mut antinode_b) =
                    ((b.0 + v.0, b.1 + v.1), (a.0 - v.0, a.1 - v.1));
                while antinode_a.0 >= 0 && antinode_a.0 < x && antinode_a.1 >= 0 && antinode_a.1 < y
                {
                    antinodes.insert(antinode_a);
                    antinode_a.0 += v.0;
                    antinode_a.1 += v.1;
                }
                while antinode_b.0 >= 0 && antinode_b.0 < x && antinode_b.1 >= 0 && antinode_b.1 < y
                {
                    antinodes.insert(antinode_b);
                    antinode_b.0 -= v.0;
                    antinode_b.1 -= v.1;
                }
            })
        });
    });
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{solve1, solve2};
    const INPUT_PATH: &str = "inputs/d8p1";
    const TEST_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn solve1_test() {
        assert_eq!(solve1(TEST_INPUT), 14);
    }

    #[test]
    fn solve1_real() {
        let input = fs::read_to_string(INPUT_PATH).expect("input file is present");
        assert_eq!(solve1(&input), 390);
    }

    #[test]
    fn solve2_test() {
        assert_eq!(solve1(TEST_INPUT), 14);
    }

    #[test]
    fn solve2_real() {
        let input = fs::read_to_string(INPUT_PATH).expect("input file is present");
        assert_eq!(solve2(&input), 581941094529163);
    }
}
