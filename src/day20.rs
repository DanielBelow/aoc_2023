use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

const LOW: usize = 0;
const HIGH: usize = 1;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ModuleKind {
    // High: nothing
    // Low:
    //  - off -> on + high
    //  - on  -> off + low
    FlipFlop(bool),

    // Remember all inputs (default low)
    // all high -> low
    // else     -> high
    Conjunction(HashMap<String, usize>),

    Broadcast,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Module {
    id: String,
    kind: ModuleKind,
    dest_mods: Vec<String>,
}

fn parse_module(line: &str) -> Module {
    let (module, dests) = line.split_once(" -> ").expect("input");

    let kind = if module.starts_with('&') {
        ModuleKind::Conjunction(HashMap::new())
    } else if module.starts_with('%') {
        ModuleKind::FlipFlop(false)
    } else {
        ModuleKind::Broadcast
    };

    let dests = dests.split(", ").map(ToString::to_string).collect_vec();

    Module {
        id: module.trim_start_matches(['&', '%']).to_string(),
        kind,
        dest_mods: dests,
    }
}

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> Vec<Module> {
    let mut v = inp.lines().map(parse_module).collect_vec();

    for idx in 0..v.len() {
        let all_in = v
            .iter()
            .filter(|it| it.dest_mods.contains(&v[idx].id))
            .map(|it| &it.id)
            .cloned()
            .collect_vec();

        if let ModuleKind::Conjunction(map) = &mut v[idx].kind {
            // [...] they initially default to remembering a low pulse for each input
            for s in all_in {
                map.insert(s, LOW);
            }
        }
    }

    v
}

#[aoc(day20, part1)]
pub fn part1(inp: &[Module]) -> usize {
    let mut high = 0;
    let mut low = 0;

    let mut inp = inp.to_owned();
    let broadcaster = find_module("broadcaster", &inp).expect("start node");
    for _ in 0..1000 {
        let mut queue = vec![(broadcaster, LOW, None)];
        low += 1;

        while let Some((idx, signal, from)) = queue.pop() {
            let (h, l) = handle_signal(&mut inp, &mut queue, idx, signal, from);
            high += h;
            low += l;
        }
    }

    high * low
}

#[aoc(day20, part2)]
pub fn part2(inp: &[Module]) -> usize {
    let mut inp = inp.to_owned();

    // rx's sole input is the conjunction mf
    // mf's inputs are: jf, sh, bh, mz
    // -> check when they emit true (cycling)
    ["jf", "sh", "bh", "mz"]
        .iter()
        .map(|it| find_cycle_for(it, &mut inp))
        .product()
}

fn find_module(id: &str, modules: &[Module]) -> Option<usize> {
    modules.iter().position(|it| it.id.eq(id))
}

fn find_cycle_for(node: &str, inp: &mut [Module]) -> usize {
    let mut prev_cycle = 0;
    let broadcaster = find_module("broadcaster", inp).expect("start node");

    for num_presses in 0.. {
        let mut queue = vec![(broadcaster, LOW, None)];

        while let Some((idx, signal, from)) = queue.pop() {
            if signal == HIGH && from == Some(node.to_string()) {
                if prev_cycle == 0 {
                    prev_cycle = num_presses;
                } else {
                    return num_presses - prev_cycle;
                }
            }

            handle_signal(inp, &mut queue, idx, signal, from);
        }
    }

    unreachable!("cycle exists")
}

fn handle_signal(
    inp: &mut [Module],
    queue: &mut Vec<(usize, usize, Option<String>)>,
    idx: usize,
    signal: usize,
    from: Option<String>,
) -> (usize, usize) {
    let send_pulse = |cur: usize,
                      signal: usize,
                      inp: &[Module],
                      queue: &mut Vec<(usize, usize, Option<String>)>|
     -> (usize, usize) {
        let high = if signal == HIGH {
            inp[cur].dest_mods.len()
        } else {
            0
        };

        let low = inp[cur].dest_mods.len() - high;

        for dn in &inp[cur].dest_mods {
            if let Some(i) = find_module(dn, inp) {
                queue.insert(0, (i, signal, Some(inp[cur].id.clone())));
            }
        }

        (high, low)
    };

    match inp[idx].kind {
        ModuleKind::Broadcast => send_pulse(idx, signal, inp, queue),
        ModuleKind::FlipFlop(ref mut state) => {
            if signal == LOW {
                let to_send = if *state {
                    // If it was on, it turns off and sends a low pulse.
                    LOW
                } else {
                    // If it was off, it turns on and sends a high pulse.
                    HIGH
                };

                *state = !*state;

                send_pulse(idx, to_send, inp, queue)
            } else {
                (0, 0)
            }
        }
        ModuleKind::Conjunction(ref mut map) => {
            // When a pulse is received, the conjunction module first updates its memory for that input.
            // Then, if it remembers high pulses for all inputs, it sends a low pulse;
            // otherwise, it sends a high pulse.
            assert!(from.is_some());

            map.entry(from.expect("sender exists"))
                .and_modify(|it| *it = signal);

            let to_send = if map.values().all(|it| *it == HIGH) {
                LOW
            } else {
                HIGH
            };

            send_pulse(idx, to_send, inp, queue)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "broadcaster -> a, b, c\n\
                              %a -> b\n\
                              %b -> c\n\
                              %c -> inv\n\
                              &inv -> a";

    const TEST_INPUT_2: &str = "broadcaster -> a\n\
                                %a -> inv, con\n\
                                &inv -> b\n\
                                %b -> con\n\
                                &con -> output";
    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 32_000_000);
    }

    #[test]
    fn test_p1_2() {
        let gen = generate(TEST_INPUT_2);
        let res = part1(&gen);
        assert_eq!(res, 11_687_500);
    }
}
