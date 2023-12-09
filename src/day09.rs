use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day09)]
pub fn generate(inp: &str) -> Vec<Vec<isize>> {
    inp.lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<isize>().expect("number"))
                .collect_vec()
        })
        .collect_vec()
}

fn pairwise_diff(current: &[isize]) -> Vec<isize> {
    current
        .iter()
        .tuple_windows()
        .map(|(l, r)| *r - *l)
        .collect_vec()
}

fn predict(row: &[isize]) -> isize {
    let mut hist = vec![row.to_vec()];

    while !hist.last().expect("non-empty").iter().all(|it| *it == 0) {
        let diff = pairwise_diff(hist.last().expect("non-empty"));
        hist.push(diff);
    }

    hist.last_mut().expect("non-empty").push(0);

    for idx in (0..hist.len() - 1).rev() {
        let cur_last = *hist[idx].last().expect("non-empty");
        let diff = *hist[idx + 1].last().expect("non-empty");
        hist[idx].push(cur_last + diff);
    }

    *hist[0].last().expect("non-empty")
}

#[aoc(day09, part1)]
pub fn part1(inp: &[Vec<isize>]) -> isize {
    let inp = inp.to_owned();

    let mut result = 0;

    for history in &inp {
        result += predict(history);
    }

    result
}

#[aoc(day09, part2)]
pub fn part2(inp: &[Vec<isize>]) -> isize {
    let inp = inp
        .to_owned()
        .iter_mut()
        .map(|r| r.iter().rev().copied().collect_vec())
        .collect_vec();

    part1(&inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15\n\
                              1 3 6 10 15 21\n\
                              10 13 16 21 30 45";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 114);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 2);
    }
}
