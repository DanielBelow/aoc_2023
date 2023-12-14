use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use std::collections::HashSet;

#[aoc_generator(day14)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|line| line.chars().collect()).collect_vec()
}

fn tilt_north(inp: &mut Vec<Vec<char>>) {
    for (y, x) in iproduct!(0..inp.len(), 0..inp[0].len()) {
        if inp[y][x] == 'O' {
            let mut top_most = y;
            while top_most > 0 && inp[top_most - 1][x] == '.' {
                inp[top_most][x] = '.';
                top_most -= 1;
                inp[top_most][x] = 'O';
            }
        }
    }
}

fn tilt_west(inp: &mut Vec<Vec<char>>) {
    for (y, x) in iproduct!(0..inp.len(), 0..inp[0].len()) {
        if inp[y][x] == 'O' {
            let mut left_most = x;
            while left_most > 0 && inp[y][left_most - 1] == '.' {
                inp[y][left_most] = '.';
                left_most -= 1;
                inp[y][left_most] = 'O';
            }
        }
    }
}

fn tilt_south(inp: &mut Vec<Vec<char>>) {
    for (y, x) in iproduct!((0..inp.len()).rev(), 0..inp[0].len()) {
        if inp[y][x] == 'O' {
            let mut south_most = y;
            while south_most < inp.len() - 1 && inp[south_most + 1][x] == '.' {
                inp[south_most][x] = '.';
                south_most += 1;
                inp[south_most][x] = 'O';
            }
        }
    }
}

fn tilt_east(inp: &mut Vec<Vec<char>>) {
    for (y, x) in iproduct!(0..inp.len(), (0..inp[0].len()).rev()) {
        if inp[y][x] == 'O' {
            let mut right_most = x;
            while right_most < inp[y].len() - 1 && inp[y][right_most + 1] == '.' {
                inp[y][right_most] = '.';
                right_most += 1;
                inp[y][right_most] = 'O';
            }
        }
    }
}

#[aoc(day14, part1)]
pub fn part1(inp: &[Vec<char>]) -> usize {
    let mut inp = inp.to_owned();
    tilt_north(&mut inp);
    calculate_load(&inp)
}

fn calculate_load(inp: &[Vec<char>]) -> usize {
    let mut result = 0;

    for (idx, row) in inp.iter().enumerate() {
        result += (inp.len() - idx) * row.iter().filter(|&it| *it == 'O').count();
    }

    result
}

fn simulate_round(inp: &mut Vec<Vec<char>>) {
    tilt_north(inp);
    tilt_west(inp);
    tilt_south(inp);
    tilt_east(inp);
}

#[aoc(day14, part2)]
pub fn part2(inp: &[Vec<char>]) -> usize {
    let mut prev_round = inp.to_owned();

    let mut cache = HashSet::new();

    let mut cycle_at = 0;

    for idx in 0usize..1_000_000_000 {
        simulate_round(&mut prev_round);

        if !cache.insert(prev_round.clone()) {
            if cycle_at == 0 {
                cycle_at = idx;

                // new start point to find the cycle length
                cache.clear();
                cache.insert(prev_round.clone());
            } else {
                let cycle_len = idx - cycle_at;

                // leftover iterations
                let rem = (1_000_000_000 - idx - 1) % cycle_len;
                for _ in 0..rem {
                    simulate_round(&mut prev_round);
                }

                return calculate_load(&prev_round);
            }
        }
    }

    unreachable!("No result found for p2")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "O....#....\n\
                              O.OO#....#\n\
                              .....##...\n\
                              OO.#O....O\n\
                              .O.....O#.\n\
                              O.#..O.#.#\n\
                              ..O..#O..O\n\
                              .......O..\n\
                              #....###..\n\
                              #OO..#....";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 136);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 64);
    }
}
