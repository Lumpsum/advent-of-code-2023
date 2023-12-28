use std::collections::HashMap;

use itertools::Itertools;

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
    fn process_pulse(&mut self, pulse: Pulse, sender: &'a str) -> Option<Vec<(&'a str, Pulse)>> {
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
