use std::str::FromStr;

use itertools::Itertools;

pub struct Mirror {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>
}

impl FromStr for Mirror {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Vec<char>> = Vec::new();
        let mut columns: Vec<Vec<char>> = Vec::new();

        for (idx, row) in s.lines().enumerate() {
            let characters = row.chars();
            rows.push(characters.collect_vec());
            let mut new_vec = match columns.len() < idx + 1 {
                true => Vec::new(),
                false => columns[idx]
            };
            new_vec.push(characters.into_iter().nth(idx).unwrap());
            columns.insert(idx, new_vec)
        }
        Ok(Self { rows: rows, columns: columns })
    }
}
