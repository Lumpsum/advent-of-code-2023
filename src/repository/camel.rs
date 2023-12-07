use std::collections::HashMap;
use std::iter::zip;
use std::cmp::Ordering;


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub enum HandType {
    High,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

impl HandType {
    fn add_integer(&self, value: i32) -> HandType {
        match (self, value) {
            (HandType::FourOfAKind, 1) => HandType::FiveOfAKind, 
            (HandType::ThreeOfAKind, 2) => HandType::FiveOfAKind, 
            (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind, 
            (HandType::TwoPair, 1) => HandType::FullHouse, 
            (HandType::OnePair, 3) => HandType::FiveOfAKind, 
            (HandType::OnePair, 2) => HandType::FourOfAKind, 
            (HandType::OnePair, 1) => HandType::ThreeOfAKind,
            (HandType::High, 4) => HandType::FiveOfAKind,
            (HandType::High, 3) => HandType::FourOfAKind,
            (HandType::High, 2) => HandType::ThreeOfAKind,
            (HandType::High, 1) => HandType::OnePair,
            (v, 0) => *v,
            (_, _) => HandType::High
        }
    }
}

impl From<(&str, bool)> for HandType {
    fn from(value: (&str, bool)) -> Self {
        let s = value.0;
        let joker = value.1;
        let mut letter_counts: HashMap<char, i32> = HashMap::new();

        let char_vec: Vec<char> = s.to_lowercase().chars().collect();
        for char in char_vec {
            *letter_counts.entry(char).or_insert(0) += 1;
        }

        let mut letter_counts_ = letter_counts.clone();
        let mut j = 0;
        if joker {
            j = *letter_counts_.get(&'j').unwrap_or(&0);
            letter_counts_.remove(&'j');
        }
        let invert_letter_counts: HashMap<i32, char> = letter_counts_.iter().map(|(k, v)| (*v, *k)).collect();
        if invert_letter_counts.is_empty() {
            return Self::FiveOfAKind
        }

        match (
            invert_letter_counts.contains_key(&5),
            invert_letter_counts.contains_key(&4),
            invert_letter_counts.contains_key(&3),
            invert_letter_counts.contains_key(&2)
        ) {
            (true, _, _, _) => return Self::FiveOfAKind,
            (_, true, _, _) => {
                return Self::add_integer(&Self::FourOfAKind, j)
            },
            (_, _, true, true) => return Self::FullHouse,
            (_, _, true, _) => {
                return Self::add_integer(&Self::ThreeOfAKind, j)
            },
            (_, _, _, true) => {
                match letter_counts_.iter().filter(|(_, &v)| (v == 2)).collect::<HashMap<&char, &i32>>().len() {
                    2 => {
                        return Self::add_integer(&Self::TwoPair, j)
                    },
                    _ => {
                        return Self::add_integer(&Self::OnePair, j)
                    }
                }
            },
            _ => {
                return Self::add_integer(&Self::High, j)
            }
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct Hand<'a> {
    pub cards: &'a str,
    pub strength: u32,
    pub hand_type: HandType,
    joker: bool
}

impl<'a> From<(&'a str, bool)> for Hand<'a> {
    fn from(value: (&'a str, bool)) -> Self {
        let mut split = value.0.split(" ");
        let cards = split.nth(0).unwrap();
        let strength = split.nth(0).unwrap().parse().unwrap();

        let hand_type = HandType::from((cards, value.1));

        Self { cards: cards, strength: strength, hand_type: hand_type, joker: value.1 }
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (char_one, char_two) in zip(self.cards.chars(), other.cards.chars()) {
                    match (char_one.is_digit(10), char_two.is_digit(10)) {
                        (true, false) => {
                            if self.joker && char_two == 'J' {
                                return Ordering::Greater
                            };
                            return Ordering::Less
                        },
                        (false, true) => {
                            if self.joker && char_one == 'J' {
                                return Ordering::Less
                            };
                            return Ordering::Greater
                        },
                        (true, true) => {
                            match char_one.cmp(&char_two) {
                                Ordering::Equal => continue,
                                v => return v
                            }
                        }
                        _ => {
                            if char_one == char_two {
                                continue
                            }
                            match (char_one, char_two, self.joker) {
                                ('A', _, _) => return Ordering::Greater,
                                (_, 'A', _) => return Ordering::Less,
                                ('K', _, _) => return Ordering::Greater,
                                (_, 'K', _) => return Ordering::Less,
                                ('Q', _, _) => return Ordering::Greater,
                                (_, 'Q', _) => return Ordering::Less,
                                ('T', _, true) => return Ordering::Greater,
                                (_, 'T', true) => return Ordering::Less,
                                ('J', _, false) => return Ordering::Greater,
                                (_, 'J', false) => return Ordering::Less,
                                _ => continue
                            }
                        }
                    }
                }
                return Ordering::Equal
            },
            v => return v
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
