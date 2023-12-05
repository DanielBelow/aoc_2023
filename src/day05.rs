use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Relation {
    None,
    FullyContains,
    IsFullyContained,
    BeginOverlaps,
    EndOverlaps,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct NumberRange {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

impl NumberRange {
    const fn end(&self) -> usize {
        self.source_start + self.length
    }

    const fn contains(&self, value: usize) -> bool {
        value >= self.source_start && value <= self.end()
    }

    const fn check_relation(&self, (start, end): (usize, usize)) -> Relation {
        if self.source_start <= start && end <= self.end() {
            Relation::FullyContains
        } else if start <= self.source_start && self.end() <= end {
            Relation::IsFullyContained
        } else if self.end() >= end && start < self.source_start && self.source_start < end {
            Relation::BeginOverlaps
        } else if self.source_start <= start && start < self.end() && self.end() < end {
            Relation::EndOverlaps
        } else {
            Relation::None
        }
    }

    const fn map_number(&self, value: usize) -> usize {
        assert!(self.contains(value));
        value - self.source_start + self.dest_start
    }
}

#[derive(Clone, Debug)]
pub struct Mapping {
    maps: Vec<NumberRange>,
}

impl Mapping {
    fn transform(&self, value: usize) -> usize {
        self.maps
            .iter()
            .find_map(|m| {
                if m.contains(value) {
                    Some(m.map_number(value))
                } else {
                    None
                }
            })
            .unwrap_or(value)
    }
}

#[derive(Clone, Debug)]
pub struct ParsedInput {
    seeds: Vec<usize>,
    mapping: Vec<Mapping>,
}

fn parse_seeds(inp: &str) -> Vec<usize> {
    let nums = inp.split(": ").nth(1).expect("seeds: prefix");

    nums.split_ascii_whitespace()
        .filter_map(|it| it.parse::<usize>().ok())
        .collect_vec()
}

#[aoc_generator(day05)]
pub fn generate(inp: &str) -> ParsedInput {
    let blocks = inp.split_terminator("\n\n").collect_vec();
    let seeds = parse_seeds(blocks[0]);

    let mut mapping = Vec::new();

    for block in blocks.iter().skip(1) {
        let maps = block.lines().skip(1).fold(vec![], |mut acc, nums| {
            let parsed_nums = nums
                .split_ascii_whitespace()
                .map(|it| it.parse::<usize>().expect("number"))
                .collect_vec();
            acc.push(NumberRange {
                dest_start: parsed_nums[0],
                source_start: parsed_nums[1],
                length: parsed_nums[2],
            });
            acc
        });

        mapping.push(Mapping { maps });
    }

    ParsedInput { seeds, mapping }
}

fn map_seed(seed: usize, mappings: &[Mapping]) -> usize {
    mappings.iter().fold(seed, |acc, it| it.transform(acc))
}

#[aoc(day05, part1)]
pub fn part1(inp: &ParsedInput) -> usize {
    inp.seeds
        .iter()
        .map(|s| map_seed(*s, &inp.mapping))
        .min()
        .expect("minimum")
}

fn create_new_ranges(
    relation: Relation,
    number_range: &NumberRange,
    seed_range: (usize, usize),
) -> Vec<(usize, usize)> {
    match relation {
        Relation::FullyContains => {
            //   [..., seed_range, ... ]
            // [...., number_range, .... ]
            // number_range fully contains seed_range -> convert seed_range
            vec![(
                number_range.map_number(seed_range.0),
                number_range.map_number(seed_range.1),
            )]
        }
        Relation::IsFullyContained => {
            // [......, seed_range, ...... ]
            //    [.., number_range, .. ]
            // seed_range fully contains number_range
            // -> keep the beginning
            // -> convert number_range source to dest
            // -> keep the end
            vec![
                (seed_range.0, number_range.source_start),
                (
                    number_range.dest_start,
                    number_range.map_number(number_range.end()),
                ),
                (number_range.end(), seed_range.1),
            ]
        }
        Relation::BeginOverlaps => {
            // [..., seed_range, ... ]
            //        [..., number_range, ... ]
            // -> keep the beginning
            // -> convert number_range
            vec![
                (seed_range.0, number_range.source_start),
                (
                    number_range.dest_start,
                    number_range.map_number(seed_range.1),
                ),
            ]
        }
        Relation::EndOverlaps => {
            //        [..., seed_range, ... ]
            // [..., number_range, ... ]
            // -> convert the overlap
            // -> keep the end
            vec![
                (
                    number_range.map_number(seed_range.0),
                    number_range.map_number(number_range.end()),
                ),
                (number_range.end(), seed_range.1),
            ]
        }
        Relation::None => unreachable!("Not called with Relation::None"),
    }
}

fn calculate_seed_mapping(
    seed_range: (usize, usize),
    mapping: &[NumberRange],
) -> Option<Vec<(usize, usize)>> {
    mapping.iter().find_map(
        |number_range| match number_range.check_relation(seed_range) {
            Relation::None => None,
            rel => Some(create_new_ranges(rel, number_range, seed_range)),
        },
    )
}

#[aoc(day05, part2)]
pub fn part2(inp: &ParsedInput) -> usize {
    let mut seed_ranges = inp
        .seeds
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect_vec();

    for map in &inp.mapping {
        let mapping = &map.maps;
        seed_ranges = seed_ranges.iter().fold(vec![], |mut acc, &seed_range| {
            let new_ranges =
                calculate_seed_mapping(seed_range, mapping).unwrap_or_else(|| vec![seed_range]);
            acc.extend_from_slice(&new_ranges);
            acc
        });
    }

    seed_ranges
        .iter()
        .map(|&(start, _)| start)
        .min()
        .expect("minimum")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13\n\
                              \n\
                              seed-to-soil map:\n\
                              50 98 2\n\
                              52 50 48\n\
                              \n\
                              soil-to-fertilizer map:\n\
                              0 15 37\n\
                              37 52 2\n\
                              39 0 15\n\
                              \n\
                              fertilizer-to-water map:\n\
                              49 53 8\n\
                              0 11 42\n\
                              42 0 7\n\
                              57 7 4\n\
                              \n\
                              water-to-light map:\n\
                              88 18 7\n\
                              18 25 70\n\
                              \n\
                              light-to-temperature map:\n\
                              45 77 23\n\
                              81 45 19\n\
                              68 64 13\n\
                              \n\
                              temperature-to-humidity map:\n\
                              0 69 1\n\
                              1 0 69\n\
                              \n\
                              humidity-to-location map:\n\
                              60 56 37\n\
                              56 93 4";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 35);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 46);
    }
}
