use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct NodeInfo {
    id: String,
    dests: Vec<String>,
}

#[aoc_generator(day25)]
pub fn generate(inp: &str) -> Vec<NodeInfo> {
    let mut result = vec![];

    for line in inp.lines() {
        let (id, targets) = line.split_once(':').expect("delimiter");

        let mut dests = vec![];
        for d in targets.split_ascii_whitespace() {
            dests.push(d.trim().to_string());
        }

        result.push(NodeInfo {
            id: id.trim().to_string(),
            dests,
        });
    }

    result
}

fn collect_node_mapping(inp: &[NodeInfo]) -> HashMap<String, HashSet<String>> {
    let mut mapping = HashMap::new();

    for ni in inp {
        let v: &mut HashSet<String> = mapping.entry(ni.id.clone()).or_default();
        for dest in &ni.dests {
            if !v.contains(dest) {
                v.insert(dest.clone());
            }
        }

        for dest in &ni.dests {
            if !mapping.contains_key(dest) {
                mapping.insert(dest.clone(), HashSet::new());
            }
        }
    }

    mapping
}

fn components_product(inp: &[NodeInfo], to_cut: &[(String, String)]) -> usize {
    let mut mapping = collect_node_mapping(inp);

    for (from, to) in to_cut {
        mapping.get_mut(from).expect("exists").remove(to);
    }

    let vertices = mapping.keys().cloned().collect_vec();

    let comps = pathfinding::prelude::strongly_connected_components(&vertices, |it| {
        let mut connections = vec![];

        let outgoing = mapping
            .get(it)
            .map_or(vec![], |n| n.iter().cloned().collect_vec());
        connections.extend_from_slice(&outgoing);

        let incoming = mapping
            .iter()
            .filter(|&(_, v)| v.contains(it))
            .map(|(k, _)| k)
            .cloned()
            .collect_vec();
        connections.extend_from_slice(&incoming);

        connections
    });

    assert_eq!(comps.len(), 2);

    comps.iter().map(Vec::len).product()
}

#[aoc(day25, part1)]
pub fn part1(inp: &[NodeInfo]) -> usize {
    // graphviz and identify visually which to cut
    components_product(
        inp,
        &[
            ("ptq".to_string(), "fxn".to_string()),
            ("fbd".to_string(), "lzd".to_string()),
            ("szl".to_string(), "kcn".to_string()),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "jqt: rhn xhk nvd\n\
                              rsh: frs pzl lsr\n\
                              xhk: hfx\n\
                              cmg: qnr nvd lhk bvb\n\
                              rhn: xhk bvb hfx\n\
                              bvb: xhk hfx\n\
                              pzl: lsr hfx nvd\n\
                              qnr: nvd\n\
                              ntq: jqt hfx bvb xhk\n\
                              nvd: lhk\n\
                              lsr: lhk\n\
                              rzs: qnr cmg lsr rsh\n\
                              frs: qnr lhk lsr";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = components_product(
            &gen,
            &[
                ("pzl".to_string(), "hfx".to_string()),
                ("cmg".to_string(), "bvb".to_string()),
                ("jqt".to_string(), "nvd".to_string()),
            ],
        );
        assert_eq!(res, 54);
    }
}
