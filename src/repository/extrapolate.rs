use std::slice::Iter;

#[derive(Debug)]
pub struct History(pub Vec<i32>);


impl History {
    pub fn find_next_value(&self) -> i32 {
        let mut sequences: Vec<Vec<i32>> = vec![self.0.clone()];
        sequences = self.find_sequence(sequences[0].clone(), sequences);
        sequences.reverse();
        let mut sequences_iter = sequences.iter();
        let current_value = sequences_iter.next().unwrap().last().unwrap();
        self.find_value(sequences_iter, *current_value)
    }

    fn find_sequence(&self, sequence: Vec<i32>, mut sequences: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let new_sequence: Vec<i32> = sequence.windows(2).map(|w| w[1] - w[0]).collect();
        sequences.push(new_sequence.clone());
        if new_sequence.iter().all(|&s| s == 0) {
            return sequences
        }
        return self.find_sequence(new_sequence, sequences)
    }

    fn find_value(&self, mut sequence: Iter<'_, Vec<i32>>, current_value: i32) -> i32 {
        match sequence.next() {
            Some(v) => {
                let last_value = v.last().unwrap();
                return self.find_value(sequence, last_value + current_value)
            },
            None => return current_value
        }
    }
}
