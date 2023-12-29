use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use num::{integer::lcm, Integer};

#[derive(PartialEq, Eq, Debug)]
pub enum Pulse {
    High,
    Low
}

pub enum FlipFlopState {
    On,
    Off
}

pub trait ProcessPulse<'a> {
    fn process_pulse(&mut self, pulse: Pulse, sender: &'a str) -> Option<Vec<(&'a str, Pulse)>>;
    fn get_inputs(&mut self) -> Option<&mut HashMap<&'a str, Pulse>> {
        return None
    }
    fn get_destinations(&self) -> &Vec<&'a str>;
}

pub struct Broadcaster<'a> {
    pub destinations: Vec<&'a str>
}

impl<'a> ProcessPulse<'a> for Broadcaster<'a> {
    fn process_pulse(&mut self, _pulse: Pulse, _sender: &'a str) -> Option<Vec<(&'a str, Pulse)>> {
        Some(self.destinations.iter().map(|&d| (d, Pulse::Low)).collect_vec())
    }

    fn get_destinations(&self) -> &Vec<&'a str> {
        &self.destinations
    }
}

pub struct FlipFlop<'a> {
    pub state: FlipFlopState,
    pub destinations: Vec<&'a str>
}

impl<'a> ProcessPulse<'a> for FlipFlop<'a> {
    fn process_pulse(&mut self, pulse: Pulse, _: &'a str) -> Option<Vec<(&'a str, Pulse)>> {
        if pulse == Pulse::Low {
            match self.state {
                FlipFlopState::On => {
                    self.state = FlipFlopState::Off;
                    return Some(self.destinations.iter().map(|&d| (d, Pulse::Low)).collect_vec())
                },
                FlipFlopState::Off => {
                    self.state = FlipFlopState::On;
                    return Some(self.destinations.iter().map(|&d| (d, Pulse::High)).collect_vec())
                },
            }
        }
        None
    }

    fn get_destinations(&self) -> &Vec<&'a str> {
        &self.destinations
    }
}

pub struct Conjuction<'a> {
    pub inputs: HashMap<&'a str, Pulse>,
    pub destinations: Vec<&'a str>
}

impl<'a> ProcessPulse<'a> for Conjuction<'a> {
    fn process_pulse(&mut self, pulse: Pulse, sender: &'a str) -> Option<Vec<(&'a str, Pulse)>> {
        self.inputs.insert(sender, pulse);

        if self.inputs.values().all(|p| p == &Pulse::High) {
            return Some(self.destinations.iter().map(|&d| (d, Pulse::Low)).collect_vec())
        }
        Some(self.destinations.iter().map(|&d| (d, Pulse::High)).collect_vec())
    }

    fn get_inputs(&mut self) -> Option<&mut HashMap<&'a str, Pulse>> {
        Some(&mut self.inputs)
    }

    fn get_destinations(&self) -> &Vec<&'a str> {
        &self.destinations
    }
}


pub fn get_modules_and_edges<'a>(s: &'a str) -> (HashMap<&'a str, Box<dyn ProcessPulse + 'a>>, Vec<(&'a str, &'a str)>) {
    let mut modules: HashMap<&str, Box<dyn ProcessPulse + 'a>> = HashMap::new();
    let mut lines = s.lines().collect_vec();
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

    for edge in edges.iter() {
        if let Some(v) = modules.get_mut(edge.1) {
            if let Some(v) = v.get_inputs() {
                v.insert(edge.0, Pulse::Low);
            }
        }
    }

    (modules, edges)
}


pub fn get_pulses<'a>(n: usize, mut modules: HashMap<&str, std::boxed::Box<dyn ProcessPulse + 'a>>) -> u32 {
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for i in 0..n {
        let mut q = VecDeque::new();
        q.push_front(("button", Pulse::Low, "broadcaster"));

        while let Some((sender, pulse, key)) = q.pop_front() {
            match pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            }

            if sender == "zf" && pulse == Pulse::High {
                println!("{:?}", i);
                panic!("")
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

    low_pulses * high_pulses
}


pub fn get_first_pulse<'a>(module_names: Vec<&str>, mut modules: HashMap<&str, std::boxed::Box<dyn ProcessPulse + 'a>>) -> Vec<u64> {
    let mut result = vec![0; module_names.len()];

    let mut i = 0;

    loop {
        i += 1;
        let mut q = VecDeque::new();
        q.push_front(("button", Pulse::Low, "broadcaster"));

        while let Some((sender, pulse, key)) = q.pop_front() {
            if pulse == Pulse::High && module_names.contains(&sender) {
                let index = module_names.iter().position(|&s| s == sender).unwrap();
                if result[index] == 0 {
                    result[index] = i
                }
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

        if !result.iter().any(|&s| s == 0) {
            break;
        }
    }
    
    result
}


pub fn lcm_vec<T>(numbers: Vec<T>) -> T 
where T: Integer + Copy + Clone {
    let mut numbers_iter = numbers.iter();
    let mut current_number = *numbers_iter.next().unwrap();
    while let Some(next_number) = numbers_iter.next() {
        current_number = lcm(current_number, *next_number)
    }
    current_number
}
