use std::collections::HashMap;

use itertools::{Itertools, repeat_n};


enum GearOptions {
    OPERATIONAL,
    DAMAGED
}


#[derive(Debug)]
pub struct ConditionRecord<'a> {
    pub sequence: &'a str,
    pub damaged_springs: Vec<u32>
}

impl<'a> From<&'a str> for ConditionRecord<'a> {
    fn from(value: &'a str) -> Self {
        let mut split = value.split(" ");
        Self { 
            sequence: split.nth(0).unwrap(),
            damaged_springs: split.nth(0).unwrap().split(",").map(|s| s.parse().unwrap()).collect()
        }
    }
}


pub fn find_solutions(conditions_records: Vec<ConditionRecord>) -> u32 {
    let mut memory: HashMap<Vec<&str>, HashMap<Vec<usize>, u32>> = HashMap::new();

    for condition_record in conditions_records {
        let filtered = condition_record.sequence.split(".").filter(|&s| s != "").collect_vec();

        for seqs in filtered.into_iter().powerset() {
            if !memory.contains_key(&seqs) {
                let mut sequence_options: Vec<HashMap<Vec<usize>, u32>> = Vec::new();
                let mut i = 0;

                for seq in &seqs[..] {
                    let variable_options = seq.chars().filter(|&c| c == '?').count();
                    let l = vec![*seq];

                    if !memory.contains_key(&l) {
                        memory = add_memory(variable_options, seq, memory);
                    }
                    sequence_options.push(memory.get(&l).unwrap().clone());
                    i += 1
                }

                if seqs.len() != 1 {
                    let mut result: HashMap<Vec<usize>, u32> = HashMap::new();

                    for mut option in sequence_options.iter().multi_cartesian_product() {
                        let mut combination: Vec<usize> = Vec::new();
                        let mut value: u32 = 0;
                        
                        for x in &mut option {
                            combination.append(&mut x.0.clone());
                            value += *x.1;
                        }

                        result.insert(combination, value);
                    }

                    memory.insert(seqs, result);
                }
            }
        }
    }
    println!("{:?}", memory);
    0
}

fn get_sequence_score(sequence: &str) -> Vec<usize> {
    sequence.split(".").filter(|&s| s != "").map(|s| s.chars().count()).collect_vec()
}

fn add_memory<'a>(l: usize, seq: &'a str, mut memory: HashMap<Vec<&'a str>, HashMap<Vec<usize>, u32>>) -> HashMap<Vec<&'a str>, HashMap<Vec<usize>, u32>> {
    let v = vec![seq];
    for mut option in repeat_n(vec!['#', '.'].into_iter(), l).multi_cartesian_product() {
        let new_option: String = seq.chars().map(|mut c| {
            if c == '?' {
                c = option.pop().unwrap()
            }
            c
        }).collect_vec().into_iter().collect();
        *memory.entry(v.clone()).or_insert(HashMap::new()).entry(
            get_sequence_score(&new_option)
        ).or_insert(0) += 1;
    }
    memory
}
