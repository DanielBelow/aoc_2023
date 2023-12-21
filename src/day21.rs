use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use num::Integer;

#[aoc_generator(day21)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|it| it.chars().collect()).collect()
}

fn count_reachable(num_steps: usize, inp: &[Vec<char>]) -> usize {
    let mut start_pos = (0, 0);

    for (y, x) in iproduct!(0..inp.len(), 0..inp[0].len()) {
        if inp[y][x] == 'S' {
            start_pos = (y, x);
            break;
        }
    }

    let mut inp = inp.to_owned();
    inp[start_pos.0][start_pos.1] = '.';

    pathfinding::prelude::dfs_reach((start_pos, 0), |&((y, x), steps)| {
        let mut succs = vec![];

        if steps > num_steps {
            return succs;
        }

        // top
        if y > 0 && inp[y - 1][x] == '.' {
            succs.push(((y - 1, x), steps + 1));
        }

        // right
        if x < inp[y].len() - 1 && inp[y][x + 1] == '.' {
            succs.push(((y, x + 1), steps + 1));
        }

        // bottom
        if y < inp.len() - 1 && inp[y + 1][x] == '.' {
            succs.push(((y + 1, x), steps + 1));
        }

        // left
        if x > 0 && inp[y][x - 1] == '.' {
            succs.push(((y, x - 1), steps + 1));
        }

        succs
    })
    .filter(|(_, steps)| *steps <= num_steps && steps.is_even())
    .unique_by(|(p, _)| *p)
    .count()
}

#[allow(dead_code)]
#[allow(clippy::cast_possible_wrap)]
fn count_reachable_infinite(num_steps: usize, inp: &[Vec<char>]) -> usize {
    let mut start_pos = (0, 0);

    for (y, x) in iproduct!(0..inp.len(), 0..inp[0].len()) {
        if inp[y][x] == 'S' {
            start_pos = (y as i64, x as i64);
            break;
        }
    }

    let mut inp = inp.to_owned();
    inp[start_pos.0 as usize][start_pos.1 as usize] = '.';

    let height = inp.len() as i64;
    let width = inp[0].len() as i64;

    pathfinding::prelude::dfs_reach((start_pos, 0), |&((y, x), steps)| {
        let mut succs = vec![];

        if steps > num_steps {
            return succs;
        }

        // top
        {
            let yy = (y - 1).rem_euclid(height) as usize;
            let xx = x.rem_euclid(width) as usize;
            if inp[yy][xx] == '.' {
                succs.push(((y - 1, x), steps + 1));
            }
        }

        // right
        {
            let yy = y.rem_euclid(height) as usize;
            let xx = (x + 1).rem_euclid(width) as usize;
            if inp[yy][xx] == '.' {
                succs.push(((y, x + 1), steps + 1));
            }
        }

        // bottom
        {
            let yy = (y + 1).rem_euclid(height) as usize;
            let xx = x.rem_euclid(width) as usize;
            if inp[yy][xx] == '.' {
                succs.push(((y + 1, x), steps + 1));
            }
        }

        // left
        {
            let yy = y.rem_euclid(height) as usize;
            let xx = (x - 1).rem_euclid(width) as usize;
            if inp[yy][xx] == '.' {
                succs.push(((y, x - 1), steps + 1));
            }
        }

        succs
    })
    .filter(|(_, steps)| *steps <= num_steps && steps % 2 == num_steps % 2)
    .unique_by(|&(p, _)| p)
    .count()
}

#[aoc(day21, part1)]
pub fn part1(inp: &[Vec<char>]) -> usize {
    count_reachable(64, inp)
}

#[aoc(day21, part2)]
pub const fn part2(inp: &[Vec<char>]) -> usize {
    // let start_pos = (65, 65);
    // let x0 = count_reachable_infinite(65 + 0 * inp.len(), inp);
    // let x1 = count_reachable_infinite(65 + 1 * inp.len(), inp);
    // let x2 = count_reachable_infinite(65 + 2 * inp.len(), inp);
    // => f(65), f(65 + inp.len()), f(65 + inp.len() * 2))

    // println!("f(65 + 0 * inp.len()) = {x0}");
    // println!("f(65 + 1 * inp.len()) = {x1}");
    // println!("f(65 + 2 * inp.len()) = {x2}");
    // => f(x) = 3848 + 15276 * x + 15186 * x^2

    let num_steps = 26_501_365usize / inp.len();
    num_steps.pow(2) * 15186 + num_steps * 15276 + 3848
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...........\n\
                              .....###.#.\n\
                              .###.##..#.\n\
                              ..#.#...#..\n\
                              ....#.#....\n\
                              .##..S####.\n\
                              .##..#...#.\n\
                              .......##..\n\
                              .##.#.####.\n\
                              .##..##.##.\n\
                              ...........";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = count_reachable(6, &gen);
        assert_eq!(res, 16);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = count_reachable_infinite(6, &gen);
        assert_eq!(res, 16);

        let gen = generate(TEST_INPUT);
        let res = count_reachable_infinite(10, &gen);
        assert_eq!(res, 50);

        let gen = generate(TEST_INPUT);
        let res = count_reachable_infinite(50, &gen);
        assert_eq!(res, 1594);

        let gen = generate(TEST_INPUT);
        let res = count_reachable_infinite(100, &gen);
        assert_eq!(res, 6536);
    }
}
