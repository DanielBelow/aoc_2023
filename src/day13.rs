use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};

#[aoc_generator(day13)]
pub fn generate(inp: &str) -> Vec<Vec<Vec<char>>> {
    inp.split_terminator("\n\n")
        .map(|it| {
            it.lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec()
}

fn transpose(v: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = v.len();
    let cols = v[0].len();

    let mut transposed = vec![vec!['.'; rows]; cols];

    for (y, x) in iproduct!(0..rows, 0..cols) {
        transposed[x][y] = v[y][x];
    }

    transposed
}

fn vertical_reflections(map: &[Vec<char>], errors: usize) -> Option<usize> {
    let transp = transpose(map);
    horizontal_reflections(&transp, errors)
}

fn count_mismatches(lhs: &[char], rhs: &[char]) -> usize {
    lhs.iter()
        .zip(rhs.iter())
        .filter(|&(l, r)| *l != *r)
        .count()
}

fn find_reflection_point(map: &[Vec<char>], errors: usize) -> Option<usize> {
    for refl_point in map
        .windows(2)
        .enumerate()
        .filter(|(_, it)| count_mismatches(&it[0], &it[1]) <= errors)
        .map(|(idx, _)| idx)
    {
        let (top_slice, bottom_slice) = map.split_at(refl_point + 1);

        let total_num_mismatches = top_slice
            .iter()
            .rev()
            .zip(bottom_slice.iter())
            .map(|(l, r)| count_mismatches(l, r))
            .sum::<usize>();

        if total_num_mismatches == errors {
            return Some(refl_point);
        }
    }

    None
}

fn horizontal_reflections(map: &[Vec<char>], errors: usize) -> Option<usize> {
    find_reflection_point(map, errors).map(|it| it + 1)
}

#[aoc(day13, part1)]
pub fn part1(inp: &[Vec<Vec<char>>]) -> usize {
    inp.iter().fold(0, |acc, it| {
        acc + vertical_reflections(it, 0).unwrap_or_default()
            + 100 * horizontal_reflections(it, 0).unwrap_or_default()
    })
}

#[aoc(day13, part2)]
pub fn part2(inp: &[Vec<Vec<char>>]) -> usize {
    inp.iter().fold(0, |acc, it| {
        acc + vertical_reflections(it, 1).unwrap_or_default()
            + 100 * horizontal_reflections(it, 1).unwrap_or_default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#.##..##.\n\
                              ..#.##.#.\n\
                              ##......#\n\
                              ##......#\n\
                              ..#.##.#.\n\
                              ..##..##.\n\
                              #.#.##.#.\n\
                              \n\
                              #...##..#\n\
                              #....#..#\n\
                              ..##..###\n\
                              #####.##.\n\
                              #####.##.\n\
                              ..##..###\n\
                              #....#..#";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 405);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 400);
    }
}
