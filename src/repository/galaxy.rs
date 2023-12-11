use std::{str::FromStr, cmp::{max, min}, collections::HashSet};
use itertools::Itertools;


pub struct Space {
    galaxies: Vec<(i32, i32)>,
    empty_rows: Vec<i32>,
    empty_columns: Vec<i32>
}

impl Space {
    pub fn sum_shortest_paths(&self, expansion: i64) -> i64 {
        let mut sum = 0;

        for galaxy in self.galaxies.iter().combinations(2) {
            sum += self.shortest_path(*galaxy[0], *galaxy[1], expansion)
        }
        sum
    }

    fn shortest_path(&self, item_one: (i32, i32), item_two: (i32, i32), expansion: i64) -> i64 {
        let mut steps = 0;

        for i in min(item_one.0, item_two.0)..max(item_one.0, item_two.0) {
            steps += 1;
            if self.empty_columns.contains(&i) {
                steps += expansion
            }
        }

        for j in min(item_one.1, item_two.1)..max(item_one.1, item_two.1) {
            steps += 1;
            if self.empty_rows.contains(&j) {
                steps += expansion
            }
        }
        steps
    }
}

impl FromStr for Space {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies: Vec<(i32, i32)> = Vec::new();
        let mut x_length = 0;
        let mut y_length = 0;
        let mut x_values: HashSet<i32> = HashSet::new();
        let mut y_values: HashSet<i32> = HashSet::new();
        let mut empty_rows: Vec<i32> = Vec::new();
        let mut empty_columns: Vec<i32> = Vec::new();
        
        for (j, line) in s.lines().enumerate() {
            for (i, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        x_values.insert(i as i32);
                        y_values.insert(j as i32);
                        galaxies.push((i as i32, j as i32));
                    },
                    _ => ()
                }
                x_length = max(x_length, i);
                y_length = max(y_length, j);
            }
        }

        for i in 0..=x_length {
            match x_values.contains(&(i as i32)) {
                true => (),
                false => empty_columns.push(i as i32)
            }
        }
        
        for i in 0..=y_length {
            match y_values.contains(&(i as i32)) {
                true => (),
                false => empty_rows.push(i as i32)
            }
        }

        Ok(Self {
            galaxies: galaxies,
            empty_rows: empty_rows,
            empty_columns: empty_columns
        })
    }
}
