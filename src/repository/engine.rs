use std::{str::FromStr, collections::HashMap, sync::atomic::{AtomicUsize, Ordering}};


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Number {
    pub id: usize,
    pub value: u32
}


#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn surrounding_values(&self) -> Vec<Coordinate> {
        let mut result: Vec<Coordinate> = Vec::new();

        for i in -1..=1 {
            for j in -1..=1 {
                result.push(Coordinate { x: self.x + i, y: self.y + j })
            }
        }

        result
    }
}


#[derive(Debug)]
pub struct EngineSchematic {
    symbol_coordinates: Vec<Coordinate>,
    number_coordinates: HashMap<Coordinate, Number>,
    pub gear_coordinates: Vec<Coordinate>
}

impl EngineSchematic {
    pub fn find_valid_numbers(&self) -> Vec<&Number> {
        let mut result: Vec<&Number> = Vec::new();

        for coord in &self.symbol_coordinates[..] {
            for c in coord.surrounding_values() {
                match self.number_coordinates.get(&c) {
                    Some(n) => result.push(n),
                    None => ()
                }
            }
        }

        result
    }

    pub fn find_gears(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        
        for coord in &self.gear_coordinates[..] {
            let mut matches: Vec<&Number> = Vec::new();

            for c in coord.surrounding_values() {
                match self.number_coordinates.get(&c) {
                    Some(n) => matches.push(n),
                    None => ()
                }
            }

            matches.dedup_by(|a, b| a.id == b.id);
            if matches.clone().len() > 1 {
                let mut v: u32 = 1;

                for m in matches {
                    v *= m.value
                }

                result.push(v);
            }
        }

        result
    }
}

impl FromStr for EngineSchematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        let mut symbol_coordinates: Vec<Coordinate> = Vec::new();
        let mut number_coordinates: HashMap<Coordinate, Number> = HashMap::new();
        let mut gear_coordinates: Vec<Coordinate> = Vec::new();

        for (i, line) in s.lines().enumerate() {
            let mut current_number = String::new();
            let mut coordinates: Vec<Coordinate> = Vec::new();

            for (j, char) in line.chars().enumerate() {
                match char.to_digit(10) {
                    Some(_) => {
                        current_number.push(char);
                        coordinates.push(Coordinate { x: i as i32, y: j as i32 });

                    },
                    None => {
                        match char {
                            '.' => (),
                            v => {
                                symbol_coordinates.push(Coordinate { x: i as i32, y: j as i32 });
                                match v {
                                    '*' => gear_coordinates.push(Coordinate { x: i as i32, y: j as i32 }),
                                    _ => ()
                                }
                            }
                        }
                        if current_number != "" {
                            let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        
                            for coord in coordinates {
                                number_coordinates.insert(
                                    coord,
                                    Number { id: id, value: current_number.parse::<u32>().unwrap() }
                                );
                            }
                            current_number = String::new();
                            coordinates = Vec::new();
                        }
                    }
                }
            }

            if current_number != "" {
                let id = COUNTER.fetch_add(1, Ordering::Relaxed);

                for coord in coordinates {
                    number_coordinates.insert(
                        coord,
                        Number { id: id, value: current_number.parse::<u32>().unwrap() }
                    );
                }
            }
        }
        
        Ok(Self {
            symbol_coordinates: symbol_coordinates,
            number_coordinates: number_coordinates,
            gear_coordinates: gear_coordinates
        })
    }
}
