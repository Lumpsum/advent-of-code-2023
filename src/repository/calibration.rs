use std::str::FromStr;
use phf::phf_map;


static NUMBER_CONVERSION: phf::Map<&'static str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

static NUMBER_CONVERSION_REV: phf::Map<&'static str, u32> = phf_map! {
    "eno" => 1,
    "owt" => 2,
    "eerht" => 3,
    "ruof" => 4,
    "evif" => 5,
    "xis" => 6,
    "neves" => 7,
    "thgie" => 8,
    "enin" => 9,
};


pub struct Callibration {
    value: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCallibrationError;

impl Callibration {
    pub fn first_and_last_digit(&self) -> Option<String> {
        let chars = self.value.chars();
        let left_digit = first_digit(chars.clone()).unwrap();
        let right_digit = first_digit(chars.rev()).unwrap();
        let mut result = left_digit.to_string();
        result.push_str(&right_digit.to_string());
        Some(result)
    }

    pub fn first_and_last_digit_written(&self) -> Option<String> {
        let left_digit = first_digit_written(self.value.chars().collect(), &NUMBER_CONVERSION).unwrap();
        let right_digit = first_digit_written(self.value.chars().rev().collect(), &NUMBER_CONVERSION_REV).unwrap();
        let mut result = left_digit.to_string();
        result.push_str(&right_digit.to_string());
        Some(result)
    }
}

impl FromStr for Callibration {
    type Err = ParseCallibrationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s.to_string()
        })
    }
}


fn first_digit<'a, I>(value: I) -> Option<u32>
where
    I: Iterator<Item = char>
{
    for char in value {
        match char::to_digit(char, 10) {
            Some(i) => return Some(i),
            None => ()
        }
    }
    None
}


fn first_digit_written(value: Vec<char>, corpus: &phf::Map<&str, u32>) -> Option<u32>
{
    let vec_length = value.len();
    for i in 0..vec_length {
        match char::to_digit(value[i], 10) {
            Some(i) => return Some(i),
            None => ()
        }

        let mut current_text = String::new();
        for j in i..vec_length {
            current_text.push(value[j]);
            match corpus.get(&current_text) {
                Some(v) => return Some(*v),
                None => ()
            }
        }
    }
    None
}
