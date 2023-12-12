use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ParsedInput {
    conditions: Vec<Vec<char>>,
    records: Vec<Vec<usize>>,
}

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> ParsedInput {
    let mut conditions = vec![];
    let mut records = vec![];

    for line in inp.lines() {
        let (cond, rec) = line.split_once(' ').expect("whitespace");
        conditions.push(cond.chars().collect_vec());
        records.push(
            rec.split(',')
                .map(|it| it.parse::<usize>().expect("input"))
                .collect_vec(),
        );
    }

    ParsedInput {
        conditions,
        records,
    }
}

#[aoc(day12, part1)]
pub fn part1(inp: &ParsedInput) -> usize {
    assert_eq!(inp.conditions.len(), inp.records.len());

    let mut result = 0;

    for idx in 0..inp.conditions.len() {
        let cur_cond = &inp.conditions[idx];
        let cur_rec = &inp.records[idx];

        let mut map = HashMap::new();

        result += count_valid_combinations(cur_cond.clone(), cur_rec.clone(), &mut map);
    }

    result
}

fn is_valid_so_far(s: &[char], r: &[usize]) -> bool {
    let groups = s
        .split(|it| *it == '.')
        .filter(|it| !it.is_empty())
        .collect_vec();

    for (g, rec) in groups.iter().zip(r.iter()) {
        if g.contains(&'?') {
            if g.iter().take_while(|&c| *c == '#').count() > *rec {
                return false;
            }

            return true;
        }

        if g.len() != *rec {
            return false;
        }
    }

    let num_question_groups = groups
        .iter()
        .filter(|it| it.iter().all(|c| *c == '?'))
        .count();

    groups.len() - num_question_groups == r.len()
}

fn extract_groups(s: &[char]) -> Vec<Vec<char>> {
    s.split(|it| *it == '.')
        .filter(|it| !it.is_empty())
        .map(|it| it.iter().copied().collect_vec())
        .collect_vec()
}

fn count_valid_combinations(
    s: Vec<char>,
    r: Vec<usize>,
    cache: &mut HashMap<Vec<Vec<char>>, usize>,
) -> usize {
    let groups = extract_groups(&s);
    if let Some(cached) = cache.get(&groups) {
        return *cached;
    }

    if !is_valid_so_far(&s, &r) {
        cache.insert(groups, 0);
        return 0;
    }

    if let Some(idx) = s.iter().position(|it| *it == '?') {
        let count_with_dot = {
            let mut new_str = s.clone();
            new_str[idx] = '.';
            let res = count_valid_combinations(new_str.clone(), r.clone(), cache);

            let groups = extract_groups(&new_str);
            cache.insert(groups, res);

            res
        };

        let count_with_hash = {
            let mut new_str = s;
            new_str[idx] = '#';
            let res = count_valid_combinations(new_str.clone(), r, cache);

            let groups = extract_groups(&new_str);
            cache.insert(groups, res);

            res
        };

        return count_with_dot + count_with_hash;
    }

    cache.insert(groups, 1);
    1
}

#[aoc(day12, part2)]
pub fn part2(inp: &ParsedInput) -> usize {
    assert_eq!(inp.conditions.len(), inp.records.len());

    let mut result = 0;

    for idx in 0..inp.conditions.len() {
        let cur_cond = &inp.conditions[idx];
        let cur_rec = &inp.records[idx];

        let combs = join_with_separator(cur_cond, cur_cond);
        let new_recs = cur_rec.repeat(5);

        let mut cache = HashMap::new();
        result += count_valid_combinations(combs, new_recs, &mut cache);
    }

    result
}

fn join_with_separator(v: &[char], repeat: &[char]) -> Vec<char> {
    let mut result = v.to_vec();
    result.push('?');
    result.extend_from_slice(repeat);
    result.push('?');
    result.extend_from_slice(repeat);
    result.push('?');
    result.extend_from_slice(repeat);
    result.push('?');
    result.extend_from_slice(repeat);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "???.### 1,1,3\n\
                              .??..??...?##. 1,1,3\n\
                              ?#?#?#?#?#?#?#? 1,3,1,6\n\
                              ????.#...#... 4,1,1\n\
                              ????.######..#####. 1,6,5\n\
                              ?###???????? 3,2,1";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 21);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 525_152);
    }
}
