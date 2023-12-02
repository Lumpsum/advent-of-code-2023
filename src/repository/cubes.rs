use std::str::FromStr;


enum Color {
    Blue,
    Green,
    Red
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            "red" => Ok(Self::Red),
            _ => Err(())
        }
    }
}

#[derive(PartialEq)]
pub struct CubeSet {
    pub blue: i32,
    pub red: i32,
    pub green: i32
}

impl CubeSet {
    fn fits(&self, cubeset: &CubeSet) -> bool {
        if self.blue <= cubeset.blue && self.red <= cubeset.red && self.green <= cubeset.green {
            return true
        }
        false
    }

    pub fn power(&self) -> u32 {
        return (self.blue * self.red * self.green).try_into().unwrap()
    }
}

impl FromStr for CubeSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        for grab in s.split(",") {
            let cube = grab.trim().split(" ").collect::<Vec<&str>>();
            match Color::from_str(cube[1]) {
                Ok(c) => {
                    match c {
                        Color::Blue => blue = cube[0].parse::<i32>().unwrap(),
                        Color::Green => green = cube[0].parse::<i32>().unwrap(),
                        Color::Red => red = cube[0].parse::<i32>().unwrap(),
                    }
                },
                Err(_) => return Err(()),
            }
        }

        Ok(Self { blue: blue, red: red, green: green })
    }
}


pub struct Game {
    pub id: u32,
    rounds: Vec<CubeSet>
}

impl Game {
    pub fn validate_all_rounds(&mut self, cubeset: CubeSet) -> bool {
        for r in &self.rounds[..] {
            if !r.fits(&cubeset) {
                return false;
            }
        };
        true
    }

    pub fn minimum_set(&self) -> CubeSet {
        let mut result = CubeSet{ blue: 0, red: 0, green: 0 };
        
        for r in &self.rounds[..] {
            if r.blue > result.blue {
                result.blue = r.blue
            }
            if r.green > result.green {
                result.green = r.green
            }
            if r.red > result.red {
                result.red = r.red
            }
        }

        return result
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(":").collect();
        let id = match parts[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>() {
            Ok(v) => v,
            Err(_) => return Err(())
        };

        let mut rounds = Vec::new();
        for round in parts[1].split(";") {
            rounds.push(CubeSet::from_str(round).unwrap())
        }
        
        Ok(Self {
            id: id,
            rounds: rounds
        })
    }
}
