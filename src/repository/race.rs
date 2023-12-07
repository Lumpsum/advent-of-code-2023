

#[derive(Debug)]
pub struct Race {
    pub time: u64,
    pub distance: u64
}

impl Race {
    pub fn calculate_ways_of_beating(&self) -> u32 {
        let mut result: u32 = 0;

        for i in 1..=self.time {
            let time_driving = self.time - i;
            match (time_driving * i) > self.distance {
                true => result += 1,
                false => ()
            }
        }

        result
    }
}


pub fn extract_numbers(s: &str) -> Vec<u64> {
    s
        .split(":").nth(1).unwrap()
        .trim().split(" ")
        .filter(|&s| s != "")
        .map(|s| s.trim().parse().unwrap())
        .collect::<Vec<u64>>()
}


pub fn extract_text(s: &str) -> Vec<&str> {
    s
        .split(":").nth(1).unwrap()
        .trim().split(" ")
        .filter(|&s| s != "")
        .map(|s| s.trim())
        .collect::<Vec<&str>>()
}


pub fn create_races(x: Vec<u64>, y: Vec<u64>) -> Vec<Race> {
    let mut result: Vec<Race> = Vec::new();

    for (i, x) in x.iter().enumerate() {
        result.push(Race{
            time: *x,
            distance: y[i]
        })
    }
    result
}
