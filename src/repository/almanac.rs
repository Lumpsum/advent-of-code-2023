use std::str::FromStr;


#[derive(Debug)]
struct Range {
    source: i64,
    destination: i64,
    range: i64
}

impl Range {
    fn fits_range(&self, value: i64) -> Option<i64> {
        if (value >= self.source) && (value < self.source + self.range) {
            return Some((value + self.destination - self.source).try_into().unwrap())
        }
        None
    }
}


pub struct Almanac {
    seeds: Vec<i64>,
    transforms: Vec<Vec<Range>>
}


impl Almanac {
    pub fn calculate_seed_locations(&mut self) -> u32 {
        let mut current_locations = self.seeds.clone();

        for transform in &self.transforms {
            for location in current_locations.iter_mut() {
                for range in transform {
                    match range.fits_range(*location) {
                        Some(v) => {
                            *location = v;
                            break
                        },
                        None => ()
                    }
                }
            }
        }
        TryInto::<u32>::try_into(*current_locations.iter().min().unwrap()).unwrap()
    }
}


impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("\n\n");

        let seeds: Vec<i64> = split.nth(0).unwrap()
            .split(": ").nth(1).unwrap()
            .split(" ").map(|s| s.parse().unwrap())
            .collect();
        
        let mut transforms = Vec::new();
        for s in split {
            let mut map = Vec::new();
            let mut value = s.split("\n");
            value.next().unwrap();
            for v in value {
                let digits: Vec<i64> = v.split(" ").map(|s| s.parse().unwrap()).collect();
                map.push(Range{ source: digits[1], destination: digits[0], range: digits[2] });
            }
            transforms.push(map)
        }

        Ok(Self{ seeds: seeds, transforms: transforms })
    }
}
