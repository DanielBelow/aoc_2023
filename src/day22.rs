use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[display("{x},{y},{z}")]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Display, FromStr, PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[display("{from}~{to}")]
pub struct Brick {
    from: Point,
    to: Point,
}

#[aoc_generator(day22)]
pub fn generate(inp: &str) -> Vec<Brick> {
    inp.lines()
        .map(|it| it.parse::<Brick>().expect("input"))
        .collect()
}

#[allow(clippy::suspicious_operation_groupings)]
fn collides_with_any(cur: usize, next_pos: Brick, bricks: &[Brick]) -> bool {
    bricks.iter().enumerate().any(|(idx, it)| {
        idx != cur
            && next_pos.from.x <= it.to.x
            && next_pos.to.x >= it.from.x
            && next_pos.from.y <= it.to.y
            && next_pos.to.y >= it.from.y
            && next_pos.from.z <= it.to.z
            && next_pos.to.z >= it.from.z
    })
}

fn simulate_fall(bricks: &mut [Brick]) -> usize {
    let mut number_falling = 0;

    for i in 0..bricks.len() {
        let cur = bricks[i];
        if cur.from.z == 1 || cur.to.z == 1 {
            continue;
        }

        let mut next_pos = cur;
        next_pos.from.z -= 1;
        next_pos.to.z -= 1;

        if !collides_with_any(i, next_pos, bricks) {
            bricks[i] = next_pos;
            number_falling += 1;
        }
    }

    number_falling
}

fn fall_initial(inp: &[Brick]) -> Vec<Brick> {
    let mut inp = inp
        .iter()
        .sorted_by_key(|it| it.from.z.min(it.to.z))
        .copied()
        .collect_vec();

    loop {
        if simulate_fall(&mut inp) == 0 {
            break;
        }
    }

    inp
}

#[aoc(day22, part1)]
pub fn part1(inp: &[Brick]) -> usize {
    let inp = fall_initial(inp);

    (0..inp.len()).fold(0, |acc, it| {
        let mut rem = inp.clone();
        rem.remove(it);
        acc + usize::from(simulate_fall(&mut rem) == 0)
    })
}

#[aoc(day22, part2)]
pub fn part2(inp: &[Brick]) -> usize {
    let inp = fall_initial(inp);

    (0..inp.len()).fold(0, |acc, it| {
        let mut rem = inp.clone();
        rem.remove(it);
        acc + simulate_fall(&mut rem)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1,0,1~1,2,1\n\
                              0,0,2~2,0,2\n\
                              0,2,3~2,2,3\n\
                              0,0,4~0,2,4\n\
                              2,0,5~2,2,5\n\
                              0,1,6~2,1,6\n\
                              1,1,8~1,1,9";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 7);
    }
}
