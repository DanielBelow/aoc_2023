use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn numbers_list<'a>(prefix: &str, line: &'a str) -> Vec<&'a str> {
    line.strip_prefix(prefix)
        .expect("prefix")
        .trim()
        .split_ascii_whitespace()
        .filter(|it| !it.is_empty())
        .collect_vec()
}

fn as_numbers(inp: &[&str]) -> Vec<usize> {
    inp.iter()
        .map(|it| it.parse::<usize>().expect("number"))
        .collect_vec()
}

fn append_combined_time(times: &[&str], res: &mut Vec<usize>) {
    let comb = times.join("").parse::<usize>().expect("number");
    res.push(comb);
}

#[aoc_generator(day06)]
pub fn generate(inp: &str) -> Vec<(usize, usize)> {
    let mut lines = inp.lines();

    let time_line = lines.next().expect("first line");
    let times_str = numbers_list("Time:", time_line);

    let mut times = as_numbers(&times_str);
    append_combined_time(&times_str, &mut times);

    let dist_line = lines.next().expect("second line");
    let dists_str = numbers_list("Distance:", dist_line);

    let mut dists = as_numbers(&dists_str);
    append_combined_time(&dists_str, &mut dists);

    times
        .iter()
        .zip(dists.iter())
        .map(|(l, r)| (*l, *r))
        .collect_vec()
}

fn ways_to_beat_record((time, dist): &(usize, usize)) -> usize {
    (1..*time).fold(0, |acc, held| {
        acc + usize::from(held * (time - held) > *dist)
    })
}

#[aoc(day06, part1)]
pub fn part1(inp: &[(usize, usize)]) -> usize {
    let (_, inp) = inp.split_last().expect("non empty");
    inp.iter().fold(1, |acc, time_and_dist| {
        acc * ways_to_beat_record(time_and_dist)
    })
}

#[aoc(day06, part2)]
pub fn part2(inp: &[(usize, usize)]) -> usize {
    let (last, _) = inp.split_last().expect("non empty");
    ways_to_beat_record(last)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30\n\
                              Distance:  9  40  200";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 288);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 71503);
    }
}
