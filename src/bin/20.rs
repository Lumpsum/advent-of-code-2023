use std::collections::{HashMap, VecDeque};

use advent_of_code::repository::pulse::{ProcessPulse, Broadcaster, FlipFlop, FlipFlopState, Conjuction, Pulse};
use itertools::Itertools;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut modules: HashMap<&str, Box<dyn ProcessPulse>> = HashMap::new();
    let mut lines = input.lines().collect_vec();
    lines.sort();
    let mut edges = Vec::new();
    for line in lines.into_iter().rev() {
        let mut split = line.split(" -> ");
        let module = split.nth(0).unwrap();
        let destinations = split.nth(0).unwrap().split(", ").collect_vec();

        match module {
            "broadcaster" => modules.insert("broadcaster", Box::new(Broadcaster{ destinations: destinations })),
            _ => {
                let mut value = module.chars();
                let first_value = value.next().unwrap();

                match first_value {
                    '%' => {
                        for d in destinations.clone() {
                            edges.push((module.rsplit_once("%").unwrap().1, d));
                        };
                        modules.insert(module.rsplit_once("%").unwrap().1, Box::new(FlipFlop{ state: FlipFlopState::Off, destinations: destinations }))
                    },
                    '&' => {
                        for d in destinations.clone() {
                            edges.push((module.rsplit_once("&").unwrap().1, d));
                        };
                        modules.insert(module.rsplit_once("&").unwrap().1, Box::new(Conjuction{ inputs: HashMap::new(), destinations: destinations }))
                    },
                    _ => continue
                }
            }
        };
    };

    for edge in edges.into_iter() {
        if let Some(v) = modules.get_mut(edge.1) {
            if let Some(v) = v.get_inputs() {
                v.insert(edge.0, Pulse::Low);
            }
        }
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let mut q = VecDeque::new();
        q.push_front(("button", Pulse::Low, "broadcaster"));

        while let Some((sender, pulse, key)) = q.pop_front() {
            match pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            }

            let module = match modules.get_mut(key) {
                Some(v) => v,
                None => continue
            };
            match module.process_pulse(pulse, sender) {
                Some(v) => {
                    for d in v.into_iter() {
                        q.push_back((key, d.1, d.0))
                    }
                },
                None => ()
            }
        }
    }

    Some(low_pulses * high_pulses)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
