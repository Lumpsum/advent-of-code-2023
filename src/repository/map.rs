use std::collections::HashMap;
use num::integer::lcm;


#[derive(Debug)]
struct Destination<'a> {
    left: &'a str,
    right: &'a str
}


#[derive(Debug)]
pub struct Maps<'a> {
    route: Vec<char>,
    maps: HashMap<&'a str, Destination<'a>>,
    starter_maps: Vec<&'a str>
}

impl Maps<'_> {
    pub fn find_single_route(&self) -> u32 {
        let route_length = self.route.clone().len();
        let mut z_found: bool = false;
        let mut current_map = "AAA";
        let mut steps: u32 = 0;
        while !z_found {
            for i in 0..route_length {
                let step = self.route[i];
                let map = match step {
                    'R' => self.maps.get(current_map).unwrap().right,
                    'L' => self.maps.get(current_map).unwrap().left,
                    _ => ""
                };
                match map {
                    "ZZZ" => z_found = true,
                    v => current_map = v
                }
                steps += 1
            }
        }
        steps
    }

    pub fn find_simultaneous_route(&self) -> u64 {
        let route_length = self.route.clone().len();
        let mut path_lengths: Vec<u64> = Vec::new();

        let mut steps: u64 = 0;
        for current_map in self.starter_maps.clone().iter_mut() {
            steps = 0;
            'outer: loop {
                for i in 0..route_length {
                    let step = self.route[i];

                    let map = match step {
                        'R' => self.maps.get(current_map).unwrap().right,
                        'L' => self.maps.get(current_map).unwrap().left,
                        _ => ""
                    };
                    *current_map = map;
                    steps += 1;

                    if current_map.ends_with("Z") {
                        path_lengths.push(steps);
                        break 'outer;
                    }
                }
            }
        }

        self.lcm(path_lengths)
    }

    fn lcm(&self, numbers: Vec<u64>) -> u64 {
        let mut numbers_iter = numbers.iter();
        let mut current_number = *numbers_iter.next().unwrap();
        while let Some(next_number) = numbers_iter.next() {
            current_number = lcm(current_number, *next_number)
        }
        current_number
    }
}

impl<'a> From<&'a str> for Maps<'a> {
    fn from(value: &'a str) -> Self {
        let mut s = value.split("\r\n\r\n");
        let route = s.nth(0).unwrap();
        let maps_text = s.nth(0).unwrap();

        let mut maps: HashMap<&str, Destination> = HashMap::new();
        let mut starter_maps: Vec<&str> = Vec::new();
        for map in maps_text.lines() {
            let mut map_split = map.split(" = ");
            let map_name = map_split.nth(0).unwrap();
            match map_name.ends_with("A") {
                true => starter_maps.push(map_name),
                false => ()
            }
            let mut destination = map_split.nth(0).unwrap().chars();
            destination.next();
            destination.next_back();
            let mut split_destination = destination.as_str().split(", ");
            maps.insert(map_name, Destination { left: split_destination.nth(0).unwrap(), right: split_destination.nth(0).unwrap() });
        }
        Self{
            route: route.chars().collect(),
            maps: maps,
            starter_maps: starter_maps
        }
    }
}
