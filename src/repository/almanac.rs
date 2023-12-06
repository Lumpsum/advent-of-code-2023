use std::str::FromStr;


#[derive(Debug)]
struct Range {
    source: i64,
    destination: i64,
    range: i64
}

impl Range {
    fn fits_range(&self, value: i64) -> bool {
        if (value >= self.source) && (value < self.source + self.range) {
            return true
        }
        false
    }
}


#[derive(Debug, Clone, Copy)]
struct SeedRange {
    min: i64,
    max: i64
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
                        true => {
                            *location = (*location + range.destination - range.source).try_into().unwrap();
                            break
                        },
                        false => ()
                    }
                }
            }
        }

        TryInto::<u32>::try_into(*current_locations.iter().min().unwrap()).unwrap()
    }

    pub fn calculate_seed_range_locations(&mut self) -> u32 {
        let mut seed_ranges: Vec<SeedRange> = self.seeds.clone()
            .chunks(2)
            .map(|s| SeedRange{ min: s[0], max: s[0] + s[1] - 1 })
            .collect();

        for transform in &self.transforms {
            let mut new_seed_ranges: Vec<SeedRange> = Vec::new();

            for range in transform {
                let mut new_seed_range: Vec<SeedRange> = Vec::new();
                for seed_range in seed_ranges.iter() {
                    if seed_range.min < range.source && seed_range.max >= range.source + range.range {
                        new_seed_range.push(SeedRange { min: seed_range.min, max: range.source - 1 });
                        new_seed_range.push(SeedRange { min: range.source + range.range, max: seed_range.max });
                        new_seed_ranges.push(SeedRange { min: range.destination, max: range.destination + range.range - 1 });
                        continue
                    }

                    (new_seed_range, new_seed_ranges) = self.match_seed_range_fit(range, seed_range, new_seed_ranges, new_seed_range);
                }
                seed_ranges = new_seed_range;
            }

            seed_ranges = seed_ranges.iter()
                .filter(|&s| s.min != -1 && s.max != -1)
                .map(|&s| s).collect::<Vec<SeedRange>>();
            seed_ranges.append(&mut new_seed_ranges.clone());
        }
        seed_ranges.iter().map(|s| s.min).min().unwrap().try_into().unwrap()
    }

    fn match_seed_range_fit(&self, range: &Range, seed_range: &SeedRange, mut new_seed_ranges: Vec<SeedRange>, mut new_seed_range: Vec<SeedRange>) -> (Vec<SeedRange>, Vec<SeedRange>) {
        let min_fits = range.fits_range(seed_range.min);
        let max_fits = range.fits_range(seed_range.max);
        let range_difference = range.destination - range.source;

        match (min_fits, max_fits) {
            (true, true) => {
                new_seed_ranges.push(SeedRange{ 
                    min: (seed_range.min + range_difference),
                    max: (seed_range.max + range_difference)
                });
            },
            (true, false) => {
                new_seed_ranges.push(SeedRange{ 
                    min: (seed_range.min + range_difference),
                    max: (range.destination + range.range - 1)
                });

                new_seed_range.push(SeedRange { min: range.source + range.range, max: seed_range.max });
            },
            (false, true) => {
                new_seed_ranges.push(SeedRange{ 
                    min: (range.destination),
                    max: (seed_range.max + range_difference)
                });

                new_seed_range.push(SeedRange { min: seed_range.min, max: range.source - 1 });
            },
            _ => new_seed_range.push(*seed_range)
        }
        (new_seed_range, new_seed_ranges)
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
