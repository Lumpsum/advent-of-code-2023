use std::str::FromStr;


#[derive(Debug)]
pub struct Dish {
    columns: Vec<u32>
}

impl Dish {
    pub fn tilt_north(&self) -> u32 {
        for col in &self.columns {
            for s in col.split('#') {
                let chars = s.chars();
                let char_length = chars.clone().count();
                let rocks = chars.filter(|&c| c == 'O').count();

            }
        }
        0
    }
}

impl FromStr for Dish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns: Vec<u32> = Vec::new();

        for (idx, line) in s.lines().enumerate() {
            for (jdx, char) in line.chars().enumerate() {
                let v = match char {
                    'O' => 0,
                    '.' => 1,
                    '#' => 2,
                    _ => 3
                };
                if idx == 0 {
                    columns.insert(jdx, v)
                } else {
                    columns[jdx].push_str(v);
                }
            }
        }

        Ok(Self { columns: columns })
    }
}
