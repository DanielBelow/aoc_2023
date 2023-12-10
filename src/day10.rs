use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use num::Integer;

#[aoc_generator(day10)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn find_loop(start_char: char, inp: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut inp = inp.to_owned();

    let mut animal_pos = (0, 0);
    for (y, x) in iproduct!(0..inp.len(), 0..inp[0].len()) {
        if inp[y][x] == 'S' {
            animal_pos = (y, x);
            break;
        }
    }

    inp[animal_pos.0][animal_pos.1] = start_char;

    let the_loop = pathfinding::prelude::dfs_reach(animal_pos, |&(y, x)| {
        let cur = inp[y][x];

        let mut succs = vec![];

        // top
        if y > 0 && (cur == '|' || cur == 'J' || cur == 'L') {
            let top = inp[y - 1][x];
            if top == '|' || top == '7' || top == 'F' {
                succs.push((y - 1, x));
            }
        }

        // right
        if x < inp[y].len() - 1 && (cur == 'L' || cur == 'F' || cur == '-') {
            let right = inp[y][x + 1];
            if right == '-' || right == 'J' || right == '7' {
                succs.push((y, x + 1));
            }
        }

        // bottom
        if y < inp.len() - 1 && (cur == '|' || cur == 'F' || cur == '7') {
            let bot = inp[y + 1][x];
            if bot == '|' || bot == 'L' || bot == 'J' {
                succs.push((y + 1, x));
            }
        }

        // left
        if x > 0 && (cur == '-' || cur == 'J' || cur == '7') {
            let left = inp[y][x - 1];
            if left == '-' || left == 'F' || left == 'L' {
                succs.push((y, x - 1));
            }
        }

        assert!(succs.len() < 3);

        succs
    });

    the_loop.collect_vec()
}

fn run_p1_with_start_as(start_char: char, inp: &[Vec<char>]) -> usize {
    let lp = find_loop(start_char, inp);
    lp.len().div_ceil(2)
}

#[aoc(day10, part1)]
pub fn part1(inp: &[Vec<char>]) -> usize {
    run_p1_with_start_as('J', inp)
}

fn run_p2_with_start_as(start_char: char, inp: &[Vec<char>]) -> usize {
    let lp = find_loop(start_char, inp);

    let mut inp = inp.to_owned();

    for (y, x) in iproduct!(0..inp.len(), 0..inp[0].len()) {
        if !lp.contains(&(y, x)) {
            inp[y][x] = '.';
        }

        if inp[y][x] == 'S' {
            inp[y][x] = start_char;
        }
    }

    iproduct!(0..inp.len(), 0..inp[0].len())
        .filter(|&(y, x)| inp[y][x] == '.')
        .fold(0, |acc, (y, x)| {
            acc + usize::from(count_hits(y, x, &inp).is_odd())
        })
}

#[aoc(day10, part2)]
pub fn part2(inp: &[Vec<char>]) -> usize {
    run_p2_with_start_as('J', inp)
}

fn count_hits(y: usize, x: usize, map: &[Vec<char>]) -> usize {
    map[y][x..]
        .iter()
        .filter(|&it| *it == '|' || *it == '7' || *it == 'F')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".....\n\
                              .S-7.\n\
                              .|.|.\n\
                              .L-J.\n\
                              .....";

    const TEST_INPUT_2: &str = "..F7.\n\
                                .FJ|.\n\
                                SJ.L7\n\
                                |F--J\n\
                                LJ...";

    const TEST_INPUT_P2: &str = "...........\n\
                                 .S-------7.\n\
                                 .|F-----7|.\n\
                                 .||.....||.\n\
                                 .||.....||.\n\
                                 .|L-7.F-J|.\n\
                                 .|..|.|..|.\n\
                                 .L--J.L--J.\n\
                                 ...........";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = run_p1_with_start_as('F', &gen);
        assert_eq!(res, 4);

        let gen = generate(TEST_INPUT_2);
        let res = run_p1_with_start_as('F', &gen);
        assert_eq!(res, 8);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT_P2);
        let res = run_p2_with_start_as('F', &gen);
        assert_eq!(res, 4);
    }
}
