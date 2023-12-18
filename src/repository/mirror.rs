use std::{str::FromStr, iter::zip};

use itertools::Itertools;


#[derive(Debug)]
pub struct Mirror {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>
}

impl Mirror {
    fn valid_mirror(&self, idx: usize, multiply: u32, iterable: &Vec<Vec<char>>) -> Option<u32> {
        for i in 0..idx {
            match (iterable.get(idx - 1 - i), iterable.get(idx + 2 + i)) {
                (Some(v), Some(v2)) => {
                    if v == v2 {
                        continue
                    }
                    return None
                },
                _ => ()
            }
        }
        Some((idx as u32 + 1) * multiply)
    }

    fn valid_mirror_smudge(&self, idx: usize, multiply: u32, iterable: &Vec<Vec<char>>, mut changes: usize) -> Option<u32> {
        for i in 0..idx {
            match (iterable.get(idx - 1 - i), iterable.get(idx + 2 + i)) {
                (Some(v), Some(v2)) => {
                    if v == v2 {
                        continue
                    }

                    if changes == 0 {
                        if zip(v, v2).filter(|(i, j)| i != j).count() == 1 {
                            changes = 1;
                            continue
                        }
                    }
                    return None
                },
                _ => ()
            }
        }

        if changes == 1 {
            return Some((idx as u32 + 1) * multiply)
        }
        None
    }

    pub fn find_mirror(&self) -> u32 {
        for (idx, rows) in self.rows[..].windows(2).enumerate() {
            if rows[0] == rows[1] {
                match self.valid_mirror(idx, 100, &self.rows) {
                    Some(v) => return v,
                    None => ()
                }
            }
        }
        
        for (idx, cols) in self.columns[..].windows(2).enumerate() {
            if cols[0] == cols[1] {
                match self.valid_mirror(idx, 1, &self.columns) {
                    Some(v) => return v,
                    None => ()
                }
            }
        }
        0
    }

    pub fn find_mirror_smudge(&self) -> u32 {
        for (idx, rows) in self.rows[..].windows(2).enumerate() {
            if rows[0] == rows[1] {
                match self.valid_mirror_smudge(idx, 100, &self.rows, 0) {
                    Some(v) => return v,
                    None => ()
                }
            }

            if zip(rows[0].clone(), rows[1].clone()).filter(|(i, j)| i != j).count() == 1 {
                match self.valid_mirror_smudge(idx, 100, &self.rows, 1) {
                    Some(v) => return v,
                    None => ()
                }
            }
        }
        
        for (idx, cols) in self.columns[..].windows(2).enumerate() {
            if cols[0] == cols[1] {
                match self.valid_mirror_smudge(idx, 1, &self.columns, 0) {
                    Some(v) => return v,
                    None => ()
                }
            }

            if zip(cols[0].clone(), cols[1].clone()).filter(|(i, j)| i != j).count() == 1 {
                match self.valid_mirror_smudge(idx, 1, &self.columns, 1) {
                    Some(v) => return v,
                    None => ()
                }
            }
        }
        println!("here");
        0
    }
}

impl FromStr for Mirror {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Vec<char>> = Vec::new();
        let mut columns: Vec<Vec<char>> = Vec::new();

        for (idx, row) in s.lines().enumerate() {
            let characters = row.chars().collect_vec();
            rows.push(characters.clone());

            for (jdx, char) in characters.iter().enumerate() {
                if idx == 0 {
                    columns.push(vec![*char])
                } else {
                    columns[jdx].push(*char)
                }
            }
        }
        Ok(Self { rows: rows, columns: columns })
    }
}
