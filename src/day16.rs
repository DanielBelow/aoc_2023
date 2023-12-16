use aoc_runner_derive::{aoc, aoc_generator};
use num::Complex;
use std::collections::HashSet;

const UP: Complex<i64> = Complex::new(0, -1);
const RIGHT: Complex<i64> = Complex::new(1, 0);
const DOWN: Complex<i64> = Complex::new(0, 1);
const LEFT: Complex<i64> = Complex::new(-1, 0);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Beam {
    direction: Complex<i64>,
    position: Complex<i64>,
}

impl Beam {
    fn turn_left(&mut self) {
        self.direction *= -Complex::i();
    }

    fn turn_right(&mut self) {
        self.direction *= Complex::i();
    }
}

#[aoc_generator(day16)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|it| it.chars().collect()).collect()
}

fn simulate_beam(
    beam: &mut Beam,
    inp_map: &[Vec<char>],
    result: &mut Vec<Vec<char>>,
    cache: &mut HashSet<(Complex<i64>, Complex<i64>)>,
) {
    loop {
        if cache.contains(&(beam.position, beam.direction)) {
            return;
        }

        result[beam.position.im as usize][beam.position.re as usize] = '#';

        cache.insert((beam.position, beam.direction));

        beam.position += beam.direction;

        if beam.position.im < 0
            || beam.position.re < 0
            || beam.position.im as usize >= inp_map.len()
            || beam.position.re as usize >= inp_map[0].len()
        {
            return;
        }

        let current_tile = inp_map[beam.position.im as usize][beam.position.re as usize];
        match current_tile {
            '.' => {}
            '\\' => {
                if beam.direction == DOWN || beam.direction == UP {
                    beam.turn_left();
                } else {
                    beam.turn_right();
                }
            }
            '/' => {
                if beam.direction == DOWN || beam.direction == UP {
                    beam.turn_right();
                } else {
                    beam.turn_left();
                }
            }
            '-' => {
                if beam.direction == UP || beam.direction == DOWN {
                    // split L&R
                    let mut left_beam = beam.clone();
                    left_beam.direction = LEFT;
                    simulate_beam(&mut left_beam, inp_map, result, cache);

                    let mut right_beam = beam.clone();
                    right_beam.direction = RIGHT;
                    simulate_beam(&mut right_beam, inp_map, result, cache);

                    return;
                }
            }
            '|' => {
                if beam.direction == LEFT || beam.direction == RIGHT {
                    // split U&D
                    let mut up_beam = beam.clone();
                    up_beam.direction = UP;
                    simulate_beam(&mut up_beam, inp_map, result, cache);

                    let mut down_beam = beam.clone();
                    down_beam.direction = DOWN;
                    simulate_beam(&mut down_beam, inp_map, result, cache);

                    return;
                }
            }
            _ => panic!("unknown symbol"),
        };
    }
}

fn count_energized(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .fold(0, |acc, it| acc + it.iter().filter(|&c| *c == '#').count())
}

fn run(beam: &mut Beam, inp: &[Vec<char>]) -> usize {
    let mut result = vec![vec!['.'; inp[0].len()]; inp.len()];

    let mut cache = HashSet::new();

    simulate_beam(beam, inp, &mut result, &mut cache);

    count_energized(&result)
}

#[aoc(day16, part1)]
pub fn part1(inp: &[Vec<char>]) -> usize {
    let mut beam = Beam {
        position: Complex::new(0, 0),
        direction: RIGHT,
    };

    run(&mut beam, inp)
}

#[aoc(day16, part2)]
pub fn part2(inp: &[Vec<char>]) -> usize {
    let mut result = 0;

    let width = inp[0].len() as i64;
    let height = inp.len() as i64;

    // top row
    for x in 0..width {
        let mut beam = Beam {
            position: Complex::new(x, 0),
            direction: DOWN,
        };

        result = run(&mut beam, inp).max(result);
    }

    // bottom row
    for x in 0..width {
        let mut beam = Beam {
            position: Complex::new(x, height - 1),
            direction: UP,
        };

        result = run(&mut beam, inp).max(result);
    }

    // left col
    for y in 0..height {
        let mut beam = Beam {
            position: Complex::new(0, y),
            direction: RIGHT,
        };

        result = run(&mut beam, inp).max(result);
    }

    // right col
    for y in 0..height {
        let mut beam = Beam {
            position: Complex::new(width - 1, y),
            direction: LEFT,
        };

        result = run(&mut beam, inp).max(result);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".|...\\....\n\
                              |.-.\\.....\n\
                              .....|-...\n\
                              ........|.\n\
                              ..........\n\
                              .........\\\n\
                              ..../.\\\\..\n\
                              .-.-/..|..\n\
                              .|....-|.\\\n\
                              ..//.|....";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 46);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 51);
    }
}
