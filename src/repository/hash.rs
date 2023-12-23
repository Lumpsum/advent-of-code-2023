use std::collections::HashMap;


pub struct Sequence<'a>(pub &'a str);

impl<'a> Sequence<'a> {
    pub fn solve(&self) -> usize {
        self.0.chars().fold(0 as usize, |acc, x| hash_algorithm(x, acc))
    }

    pub fn hashmap(&self, mut boxes: Boxes) -> Boxes {
        let mut chars = self.0.chars();
        let mut ascii = 0;
        let mut label = String::new();
        while let Some(char) = chars.next() {
            match char {
                '=' => {
                    let lens = chars.next().unwrap().to_digit(10).unwrap();
                    let b = boxes.0.get_mut(ascii).unwrap();
                    match b.1.insert(label.clone(), lens) {
                        Some(_) => (),
                        None => b.0.push(label.clone()),
                    }
                },  
                '-' => {
                    let b = boxes.0.get_mut(ascii).unwrap();
                    b.0.retain(|x| x != &label.as_str());
                    b.1.remove(label.as_str());
                }
                v => {
                    ascii = hash_algorithm(v, ascii);
                    label.push(v);
                }
            }
        }
        boxes
    }
}

fn hash_algorithm(char: char, current_value: usize) -> usize {
    let mut result = current_value as u32;
    result += char as u32;
    result *= 17;
    (result % 256) as usize
}


#[derive(Clone)]
pub struct Box(pub Vec<String>, pub HashMap<String, u32>);

#[derive(Clone)]
pub struct Boxes(pub Vec<Box>);
