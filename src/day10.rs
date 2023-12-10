use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use num::Integer;

#[aoc_generator(day10)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

const fn connects_right(left: char) -> bool {
    left == '-' || left == 'F' || left == 'L'
}

const fn connects_top(bot: char) -> bool {
    bot == '|' || bot == 'J' || bot == 'L'
}

const fn connects_left(right: char) -> bool {
    right == '-' || right == 'J' || right == '7'
}

const fn connects_bottom(top: char) -> bool {
    top == '|' || top == 'F' || top == '7'
}

fn find_start_pos(inp: &[Vec<char>]) -> (usize, usize) {
    for (y, x) in iproduct!(0..inp.len(), 0..inp[0].len()) {
        if inp[y][x] == 'S' {
            return (y, x);
        }
    }

    unreachable!("no start position found")
}

fn find_loop(start_pos: (usize, usize), inp: &[Vec<char>]) -> Vec<(usize, usize)> {
    let height = inp.len();
    let width = inp[0].len();

    pathfinding::prelude::dfs_reach(start_pos, |&(y, x)| {
        let cur = inp[y][x];

        let mut succs = vec![];

        // top
        if y > 0 && connects_top(cur) && connects_bottom(inp[y - 1][x]) {
            succs.push((y - 1, x));
        }

        // right
        if x < width - 1 && connects_right(cur) && connects_left(inp[y][x + 1]) {
            succs.push((y, x + 1));
        }

        // bottom
        if y < height - 1 && connects_bottom(cur) && connects_top(inp[y + 1][x]) {
            succs.push((y + 1, x));
        }

        // left
        if x > 0 && connects_left(cur) && connects_right(inp[y][x - 1]) {
            succs.push((y, x - 1));
        }

        assert!(succs.len() < 3);

        succs
    })
    .collect_vec()
}

fn run_p1_with_start_as(start_char: char, inp: &[Vec<char>]) -> usize {
    let mut inp = inp.to_owned();

    let start_pos = find_start_pos(&inp);
    inp[start_pos.0][start_pos.1] = start_char;

    let lp = find_loop(start_pos, &inp);
    lp.len().div_ceil(2)
}

fn count_hits(y: usize, x: usize, map: &[Vec<char>]) -> usize {
    map[y][x..]
        .iter()
        .filter(|&it| connects_bottom(*it))
        .count()
}

fn run_p2_with_start_as(start_char: char, inp: &[Vec<char>]) -> usize {
    let mut inp = inp.to_owned();
    let height = inp.len();
    let width = inp[0].len();

    let start_pos = find_start_pos(&inp);
    inp[start_pos.0][start_pos.1] = start_char;

    let lp = find_loop(start_pos, &inp);

    let mut cleaned_map = vec![vec!['.'; width]; height];

    for &(y, x) in &lp {
        cleaned_map[y][x] = inp[y][x];
    }

    iproduct!(0..height, 0..width)
        .filter(|&(y, x)| cleaned_map[y][x] == '.')
        .fold(0, |acc, (y, x)| {
            acc + usize::from(count_hits(y, x, &cleaned_map).is_odd())
        })
}

#[aoc(day10, part1)]
pub fn part1(inp: &[Vec<char>]) -> usize {
    run_p1_with_start_as('J', inp)
}

#[aoc(day10, part2)]
pub fn part2(inp: &[Vec<char>]) -> usize {
    run_p2_with_start_as('J', inp)
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
