use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Signal {
    Low,
    High,
}

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
    Conjunction(HashMap<usize, Signal>),

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

    Module {
        id: module.trim_start_matches(['&', '%']).to_string(),
        kind,
        dest_mods: dests.split(", ").map(ToString::to_string).collect(),
    }
}

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> Vec<Module> {
    let mut parsed = inp.lines().map(parse_module).collect_vec();

    for idx in 0..parsed.len() {
        let cur_name = &parsed[idx].id;
        let all_in = parsed
            .iter()
            .enumerate()
            .filter_map(|(index, it)| it.dest_mods.contains(cur_name).then_some(index))
            .collect_vec();

        if let ModuleKind::Conjunction(map) = &mut parsed[idx].kind {
            // [...] they initially default to remembering a low pulse for each input
            for idx in all_in {
                map.insert(idx, Signal::Low);
            }
        }
    }

    parsed
}

#[aoc(day20, part1)]
pub fn part1(inp: &[Module]) -> usize {
    let mut high = 0;
    let mut low = 0;

    let mut inp = inp.to_owned();
    let broadcaster = find_module("broadcaster", &inp).expect("start node");
    for _ in 0..1000 {
        let mut queue = VecDeque::from([(broadcaster, Signal::Low, None)]);
        low += 1;

        while let Some((idx, signal, from)) = queue.pop_front() {
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
        .filter_map(|it| find_module(it, &inp).map(|idx| find_cycle_for(idx, &mut inp)))
        .product()
}

fn find_module(id: &str, modules: &[Module]) -> Option<usize> {
    modules.iter().position(|it| it.id == id)
}

fn find_cycle_for(node: usize, inp: &mut [Module]) -> usize {
    let broadcaster = find_module("broadcaster", inp).expect("start node");

    let mut prev_cycle = 0;

    for num_presses in 0.. {
        let mut queue = VecDeque::from([(broadcaster, Signal::Low, None)]);

        while let Some((idx, signal, from)) = queue.pop_front() {
            if signal == Signal::High && from == Some(node) {
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

fn send_pulse(
    cur: usize,
    signal: Signal,
    inp: &[Module],
    queue: &mut VecDeque<(usize, Signal, Option<usize>)>,
) -> (usize, usize) {
    let dest_mods = &inp[cur].dest_mods;
    let high = if signal == Signal::High {
        dest_mods.len()
    } else {
        0
    };

    let low = dest_mods.len() - high;

    for dn in dest_mods {
        if let Some(i) = find_module(dn, inp) {
            queue.push_back((i, signal, Some(cur)));
        }
    }

    (high, low)
}

fn handle_signal(
    inp: &mut [Module],
    queue: &mut VecDeque<(usize, Signal, Option<usize>)>,
    idx: usize,
    signal: Signal,
    from: Option<usize>,
) -> (usize, usize) {
    match inp[idx].kind {
        ModuleKind::Broadcast => send_pulse(idx, signal, inp, queue),
        ModuleKind::FlipFlop(ref mut state) => {
            if signal == Signal::Low {
                let to_send = if *state {
                    // If it was on, it turns off and sends a low pulse.
                    Signal::Low
                } else {
                    // If it was off, it turns on and sends a high pulse.
                    Signal::High
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
            let sender = from.expect("sender exists");
            map.entry(sender).and_modify(|it| *it = signal);

            let to_send = if map.values().all(|it| *it == Signal::High) {
                Signal::Low
            } else {
                Signal::High
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
