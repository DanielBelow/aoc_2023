use aoc_runner_derive::{aoc, aoc_generator};
use geo::{Coord, Line};
use intersect2d::intersect;
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};
use std::ops::{Add, Mul, Sub};
use z3::ast::Ast;

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("{x}, {y}, {z}")]
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Point3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Point3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("{position} @ {velocity}")]
pub struct Hailstone {
    position: Point3D,
    velocity: Point3D,
}

#[aoc_generator(day24)]
pub fn generate(inp: &str) -> Vec<Hailstone> {
    inp.lines()
        .map(|it| it.parse::<Hailstone>().expect("input"))
        .collect()
}

#[aoc(day24, part1)]
pub fn part1(inp: &[Hailstone]) -> usize {
    const MIN: f64 = 200_000_000_000_000.0;
    const MAX: f64 = 400_000_000_000_000.0;

    count_collisions_in_boundary(MIN, MAX, inp)
}

#[aoc(day24, part2)]
#[allow(clippy::unwrap_used)]
pub fn part2(inp: &[Hailstone]) -> i64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let px0 = z3::ast::Int::new_const(&ctx, "px0");
    let py0 = z3::ast::Int::new_const(&ctx, "py0");
    let pz0 = z3::ast::Int::new_const(&ctx, "pz0");

    let vx0 = z3::ast::Int::new_const(&ctx, "vx0");
    let vy0 = z3::ast::Int::new_const(&ctx, "vy0");
    let vz0 = z3::ast::Int::new_const(&ctx, "vz0");

    let mut times = vec![];
    for i in 0..inp.len() {
        times.push(z3::ast::Int::new_const(&ctx, format!("t{i}")));
    }

    for (idx, line) in inp.iter().enumerate() {
        let px = z3::ast::Int::from_i64(&ctx, line.position.x as i64);
        let py = z3::ast::Int::from_i64(&ctx, line.position.y as i64);
        let pz = z3::ast::Int::from_i64(&ctx, line.position.z as i64);

        let vx = z3::ast::Int::from_i64(&ctx, line.velocity.x as i64);
        let vy = z3::ast::Int::from_i64(&ctx, line.velocity.y as i64);
        let vz = z3::ast::Int::from_i64(&ctx, line.velocity.z as i64);

        let lhs_x = times[idx].clone().mul(&vx).add(&px);
        let rhs_x = times[idx].clone().mul(&vx0).add(&px0);
        solver.assert(&lhs_x._eq(&rhs_x));

        let lhs_y = times[idx].clone().mul(&vy).add(&py);
        let rhs_y = times[idx].clone().mul(&vy0).add(&py0);
        solver.assert(&lhs_y._eq(&rhs_y));

        let lhs_z = times[idx].clone().mul(&vz).add(&pz);
        let rhs_z = times[idx].clone().mul(&vz0).add(&pz0);
        solver.assert(&lhs_z._eq(&rhs_z));
    }

    assert_eq!(solver.check(), z3::SatResult::Sat);
    if let Some(model) = solver.get_model() {
        return model
            .eval(&px0.add(py0).add(pz0), true)
            .and_then(|it| it.as_i64())
            .expect("solution");
    }

    unreachable!("no solution found")
}

fn count_collisions_in_boundary(min: f64, max: f64, stones: &[Hailstone]) -> usize {
    // stretched line segment start, end inside [min, max]
    let v = stones
        .iter()
        .map(|it| {
            let start = it.position;
            let end = it.position + it.velocity * max;

            Line {
                start: Coord {
                    x: start.x,
                    y: start.y,
                },
                end: Coord { x: end.x, y: end.y },
            }
        })
        .collect_vec();

    let mut res = 0;

    for c in v.iter().combinations(2) {
        if c.iter().all_equal() {
            continue;
        }

        if let Some(intersec) = intersect(c[0], c[1]) {
            let coord = intersec.single();
            if coord.x >= min && coord.x <= max && coord.y >= min && coord.y <= max {
                res += 1;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "19, 13, 30 @ -2, 1, -2\n\
                              18, 19, 22 @ -1, -1, -2\n\
                              20, 25, 34 @ -2, -2, -4\n\
                              12, 31, 28 @ -1, -2, -1\n\
                              20, 19, 15 @ 1, -5, -3";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = count_collisions_in_boundary(7f64, 27f64, &gen);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 47);
    }
}
