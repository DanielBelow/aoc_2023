use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use std::collections::HashSet;

#[aoc_generator(day11)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn find_galaxies(inp: &[Vec<char>]) -> HashSet<(usize, usize)> {
    iproduct!(0..inp.len(), 0..inp[0].len())
        .filter(|&(y, x)| inp[y][x] == '#')
        .collect()
}

fn count_empty_rows_between(row_start: usize, row_end: usize, inp: &[Vec<char>]) -> usize {
    let start = row_start.min(row_end);
    let end = row_start.max(row_end);

    inp.iter()
        .enumerate()
        .filter(|&(idx, row)| {
            let is_within_bounds = end > idx && idx > start;
            is_within_bounds && row.iter().all(|ch| *ch == '.')
        })
        .count()
}

fn count_empty_cols_between(col_start: usize, col_end: usize, inp: &[Vec<char>]) -> usize {
    let start = col_start.min(col_end);
    let end = col_start.max(col_end);

    (start + 1..end)
        .filter(|idx| inp.iter().all(|it| it[*idx] == '.'))
        .count()
}

fn shortest_paths_after_expansion(num_expansions: usize, inp: &[Vec<char>]) -> usize {
    let galaxies = find_galaxies(inp);

    let expansion_factor = num_expansions - 1;

    galaxies.iter().combinations(2).fold(0, |acc, galaxy| {
        let &(from_y, from_x) = galaxy[0];
        let &(to_y, to_x) = galaxy[1];

        let empty_rows = expansion_factor * count_empty_rows_between(from_y, to_y, inp);
        let empty_cols = expansion_factor * count_empty_cols_between(from_x, to_x, inp);

        acc + to_x.abs_diff(from_x) + to_y.abs_diff(from_y) + empty_rows + empty_cols
    })
}

#[aoc(day11, part1)]
pub fn part1(inp: &[Vec<char>]) -> usize {
    shortest_paths_after_expansion(2, inp)
}

#[aoc(day11, part2)]
pub fn part2(inp: &[Vec<char>]) -> usize {
    shortest_paths_after_expansion(1_000_000, inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...#......\n\
                              .......#..\n\
                              #.........\n\
                              ..........\n\
                              ......#...\n\
                              .#........\n\
                              .........#\n\
                              ..........\n\
                              .......#..\n\
                              #...#.....";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 374);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = shortest_paths_after_expansion(10, &gen);
        assert_eq!(res, 1030);

        let gen = generate(TEST_INPUT);
        let res = shortest_paths_after_expansion(100, &gen);
        assert_eq!(res, 8410);
    }
}
