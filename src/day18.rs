use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Clone, Debug)]
pub enum Instruction {
    #[display("U {0} (#{1})")]
    Up(i64, String),

    #[display("R {0} (#{1})")]
    Right(i64, String),

    #[display("D {0} (#{1})")]
    Down(i64, String),

    #[display("L {0} (#{1})")]
    Left(i64, String),
}

#[aoc_generator(day18)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines().map(|it| it.parse().expect("input")).collect()
}

fn fill_map(inp: &[Instruction]) -> Vec<(i64, i64)> {
    let mut map = Vec::new();

    let mut cur = (0, 0);
    map.push(cur);

    for inst in inp {
        match *inst {
            Instruction::Up(n, _) => {
                for _ in 0..n {
                    cur.0 += 1;
                    map.push(cur);
                }
            }
            Instruction::Right(n, _) => {
                for _ in 0..n {
                    cur.1 += 1;
                    map.push(cur);
                }
            }
            Instruction::Down(n, _) => {
                for _ in 0..n {
                    cur.0 -= 1;
                    map.push(cur);
                }
            }
            Instruction::Left(n, _) => {
                for _ in 0..n {
                    cur.1 -= 1;
                    map.push(cur);
                }
            }
        };
    }

    map
}

#[aoc(day18, part1)]
pub fn part1(inp: &[Instruction]) -> usize {
    let map = fill_map(inp);

    let mut area = 0;
    for i in 0..map.len() {
        let (y, x) = map[i];
        let (yn, xn) = map[(i + 1) % map.len()];

        area += (x + xn) * (y - yn);
    }

    let perim = map.len();
    1 + (area.unsigned_abs() as usize + perim) / 2
}

fn translate(i: &Instruction) -> Instruction {
    let (num, dir) = match i {
        Instruction::Right(_, s)
        | Instruction::Down(_, s)
        | Instruction::Up(_, s)
        | Instruction::Left(_, s) => s.split_at(s.len() - 1),
    };

    let num = i64::from_str_radix(num, 16).expect("hex number");

    // 0 means R, 1 means D, 2 means L, and 3 means U.
    let dgt = dir.parse::<i64>().expect("single digit");

    if dgt == 0 {
        Instruction::Right(num, String::new())
    } else if dgt == 1 {
        Instruction::Down(num, String::new())
    } else if dgt == 2 {
        Instruction::Left(num, String::new())
    } else {
        Instruction::Up(num, String::new())
    }
}

#[aoc(day18, part2)]
pub fn part2(inp: &[Instruction]) -> usize {
    let inp = inp.iter().map(translate).collect_vec();
    part1(&inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 6 (#70c710)\n\
                              D 5 (#0dc571)\n\
                              L 2 (#5713f0)\n\
                              D 2 (#d2c081)\n\
                              R 2 (#59c680)\n\
                              D 2 (#411b91)\n\
                              L 5 (#8ceee2)\n\
                              U 2 (#caa173)\n\
                              L 1 (#1b58a2)\n\
                              U 2 (#caa171)\n\
                              R 2 (#7807d2)\n\
                              U 3 (#a77fa3)\n\
                              L 2 (#015232)\n\
                              U 2 (#7a21e3)";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 62);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 952_408_144_115);
    }
}
