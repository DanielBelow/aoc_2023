use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::Complex;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Player {
    direction: Complex<i64>,
    position: Complex<i64>,
    straight: usize,
}

impl Player {
    fn turn_left(&mut self) {
        self.direction *= -Complex::i();
    }

    fn turn_right(&mut self) {
        self.direction *= Complex::i();
    }
}

#[aoc_generator(day17)]
pub fn generate(inp: &str) -> Vec<Vec<usize>> {
    inp.lines()
        .map(|it| {
            it.chars()
                .map(|it| it.to_digit(10).expect("digit") as usize)
                .collect()
        })
        .collect()
}

fn find_path<F, S>(successors: F, success: S) -> Option<usize>
where
    F: Fn(&Player) -> Vec<(Player, usize)>,
    S: Fn(&Player) -> bool,
{
    let player = Player {
        position: Complex::new(0, 0),
        direction: Complex::new(1, 0),
        straight: 1,
    };

    pathfinding::prelude::dijkstra(&player, successors, success).map(|it| it.1)
}

fn valid_positions(succs: &[Player], inp: &[Vec<usize>]) -> Vec<(Player, usize)> {
    let height = inp.len();
    let width = inp[0].len();

    succs
        .iter()
        .filter(|it| {
            it.position.im >= 0
                && it.position.re >= 0
                && (it.position.im as usize) < height
                && (it.position.re as usize) < width
        })
        .cloned()
        .map(|it| {
            let cost = inp[it.position.im as usize][it.position.re as usize];
            (it, cost)
        })
        .collect_vec()
}

const fn on_final_square(p: &Player, width: usize, height: usize) -> bool {
    p.position.im as usize == height - 1 && p.position.re as usize == width - 1
}

#[aoc(day17, part1)]
pub fn part1(inp: &[Vec<usize>]) -> usize {
    let height = inp.len();
    let width = inp[0].len();

    find_path(
        |p| {
            let mut succs = vec![];

            // straight
            if p.straight < 3 {
                let mut straight = p.clone();
                straight.position += straight.direction;
                straight.straight += 1;
                succs.push(straight);
            }

            // left
            let mut left = p.clone();
            left.turn_left();
            left.position += left.direction;
            left.straight = 1;
            succs.push(left);

            // right
            let mut right = p.clone();
            right.turn_right();
            right.position += right.direction;
            right.straight = 1;
            succs.push(right);

            valid_positions(&succs, inp)
        },
        |p| on_final_square(p, width, height),
    )
    .expect("found path")
}

#[aoc(day17, part2)]
pub fn part2(inp: &[Vec<usize>]) -> usize {
    let height = inp.len();
    let width = inp[0].len();

    find_path(
        |p| {
            let mut succs = vec![];

            // straight
            if p.straight < 10 {
                let mut straight = p.clone();
                straight.position += straight.direction;
                straight.straight += 1;
                succs.push(straight);
            }

            if p.straight >= 4 {
                // left
                let mut left = p.clone();
                left.turn_left();
                left.position += left.direction;
                left.straight = 1;
                succs.push(left);

                // right
                let mut right = p.clone();
                right.turn_right();
                right.position += right.direction;
                right.straight = 1;
                succs.push(right);
            }

            valid_positions(&succs, inp)
        },
        |p| on_final_square(p, width, height) && p.straight >= 4,
    )
    .expect("found path")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2413432311323\n\
                              3215453535623\n\
                              3255245654254\n\
                              3446585845452\n\
                              4546657867536\n\
                              1438598798454\n\
                              4457876987766\n\
                              3637877979653\n\
                              4654967986887\n\
                              4564679986453\n\
                              1224686865563\n\
                              2546548887735\n\
                              4322674655533";

    const TEST_INPUT_P2_1: &str = "111111111111\n\
                                   999999999991\n\
                                   999999999991\n\
                                   999999999991\n\
                                   999999999991";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 102);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 94);
    }

    #[test]
    fn test_p2_2() {
        let gen = generate(TEST_INPUT_P2_1);
        let res = part2(&gen);
        assert_eq!(res, 71);
    }
}
