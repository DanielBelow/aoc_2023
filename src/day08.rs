use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::Integer;
use parse_display_derive::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, PartialEq, Eq, Hash, Clone, Debug)]
#[display("{name} = ({left}, {right})")]
pub struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
pub enum Instruction {
    #[display("L")]
    Left,

    #[display("R")]
    Right,
}

#[derive(Clone, Debug)]
pub struct ParsedInput {
    insts: Vec<Instruction>,
    nodes: HashMap<Node, (Node, Node)>,
}

fn build_node_mapping(inp: &[Node]) -> HashMap<Node, (Node, Node)> {
    let mut node_map = HashMap::new();

    for node in inp {
        let left_node = inp
            .iter()
            .find(|it| it.name == node.left)
            .expect("left node exist");

        let right_node = inp
            .iter()
            .find(|it| it.name == node.right)
            .expect("right node exists");

        node_map.insert(node.clone(), (left_node.clone(), right_node.clone()));
    }

    node_map
}

#[aoc_generator(day08)]
pub fn generate(inp: &str) -> ParsedInput {
    let mut lines = inp.lines();
    let insts = lines
        .next()
        .expect("first line")
        .chars()
        .map(|it| it.to_string().parse::<Instruction>().expect("input"))
        .collect_vec();

    let nodes = lines
        .filter(|it| !it.is_empty())
        .map(|it| it.parse::<Node>().expect("node"))
        .collect_vec();

    let nodes = build_node_mapping(&nodes);

    ParsedInput { insts, nodes }
}

fn steps_until_target_node<P>(
    from: &Node,
    insts: &[Instruction],
    nodes: &HashMap<Node, (Node, Node)>,
    target_check: P,
) -> usize
where
    P: Fn(&Node) -> bool,
{
    let mut inst_iter = insts.iter().cycle();
    let mut cur_node = from;

    for step in 0.. {
        if target_check(cur_node) {
            return step;
        }

        let (left, right) = nodes.get(cur_node).expect("known node");

        cur_node = match inst_iter.next().expect("cycle") {
            Instruction::Left => left,
            Instruction::Right => right,
        };
    }

    unreachable!()
}

#[aoc(day08, part1)]
pub fn part1(inp: &ParsedInput) -> usize {
    let cur_node = inp
        .nodes
        .keys()
        .find(|it| it.name == "AAA")
        .expect("start node");

    steps_until_target_node(cur_node, &inp.insts, &inp.nodes, |it| it.name == "ZZZ")
}

#[aoc(day08, part2)]
pub fn part2(inp: &ParsedInput) -> usize {
    let start_nodes = inp.nodes.keys().filter(|it| it.name.ends_with('A'));

    start_nodes
        .map(|it| steps_until_target_node(it, &inp.insts, &inp.nodes, |it| it.name.ends_with('Z')))
        .fold(1usize, |acc, it| acc.lcm(&it))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "RL\n\
                              \n\
                              AAA = (BBB, CCC)\n\
                              BBB = (DDD, EEE)\n\
                              CCC = (ZZZ, GGG)\n\
                              DDD = (DDD, DDD)\n\
                              EEE = (EEE, EEE)\n\
                              GGG = (GGG, GGG)\n\
                              ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = "LLR\n\
                                \n\
                                AAA = (BBB, BBB)\n\
                                BBB = (AAA, ZZZ)\n\
                                ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_P2: &str = "LR\n\
                                 \n\
                                 11A = (11B, XXX)\n\
                                 11B = (XXX, 11Z)\n\
                                 11Z = (11B, XXX)\n\
                                 22A = (22B, XXX)\n\
                                 22B = (22C, 22C)\n\
                                 22C = (22Z, 22Z)\n\
                                 22Z = (22B, 22B)\n\
                                 XXX = (XXX, XXX)";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 2);

        let gen = generate(TEST_INPUT_2);
        let res = part1(&gen);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT_P2);
        let res = part2(&gen);
        assert_eq!(res, 6);
    }
}
