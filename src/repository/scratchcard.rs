use std::str::FromStr;

pub struct Scratchcard {
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>
}

impl Scratchcard {
    pub fn find_matching_numbers(&self) -> Vec<&u32> {
        self.winning_numbers.iter().filter(|&s| self.your_numbers.contains(s)).collect()
    }
}

impl FromStr for Scratchcard {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut both_numbers = s.split(":").nth(1).unwrap().split("|");
        Ok(Self {
            winning_numbers: get_numbers(both_numbers.clone().nth(0).unwrap()),
            your_numbers: get_numbers(both_numbers.nth(1).unwrap())
        })
    }
}


fn get_numbers(s: &str) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for number in s.split(" ") {
        match number.parse::<u32>() {
            Ok(v) => result.push(v),
            Err(_) => ()
        }
    }

    result
}
