use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Ace,
    Number(i32),
    Face(FaceType),
    Joker,
}

pub struct ValueIter {
    count: i32,
}

impl Iterator for ValueIter {
    type Item = Value;
    
    fn next(&mut self) -> Option<Value> {
        self.count += 1;

        if self.count == 14 { None } else { Some(Value::int_to_val(self.count)) }
    }
}

impl Value {
    fn val_to_int(v: Value) -> i32 {
        use Value::*;
        use FaceType::*;
        
        match v {
            Ace => 1,
            Number(n) => n,
            Face(f) => match f {
                Jack => 11,
                Queen => 12,
                King => 13,
            },
            Joker => {
                eprintln!("Warning: adding Jokers is undefined and will force into normal card!");
                14
            },
        }
    }
    
    fn int_to_val(n: i32) -> Value {
        use Value::*;
        use FaceType::*;

        match n {
            1 => Ace,
            2..=10 => Number(n),
            11 => Face(Jack),
            12 => Face(Queen),
            13 => Face(King),
            14 => Joker,
            _ => panic!("attempted to convert {} to a number, must be in 1..=14", n),
        }
    }
    
    pub fn values() -> ValueIter {
        ValueIter {
            count: 0,
        }
    }
}

impl Add<i32> for Value {
    type Output = Value;
    
    fn add(self, other: i32) -> Value {
        let n: i32 = (Value::val_to_int(self) + other - 1) % 13 + 1;
        Value::int_to_val(n)
    }
}

impl Sub<i32> for Value {
    type Output = Value;

    fn sub(self, other: i32) -> Value {
        let n: i32 = (Value::val_to_int(self) - other) % 13;
        let n: i32 = if n <= 0 { n + 13 } else { n };
        Value::int_to_val(n)
    }
    
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FaceType {
    Jack,
    Queen,
    King,
}

#[derive(Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    Red,
    Black,
}

#[derive(Debug)]
pub enum State {
    Hand,
    Deck,
    Play,
    Discard,
}

#[derive(Debug)]
pub struct Card {
    value: Value,
    suit: Suit,
    state: State,
}

impl Card {
    pub fn new(v: Value, s: Suit, st: State) -> Card {
        Card {
            value: v,
            suit: s,
            state: st,
        }
    }
}
/*
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(small: Value, large: Value, num_of_each: u32) -> Deck {
        let mut v: Vec<Card> = Vec::new();
        
    }
}
*/
#[cfg(test)]
mod tests {
    use crate::cards::*;
    #[test]
    fn test_card_creation() {
        println!("***************inside of a test****************");
        println!("{:?}", Card::new(Value::Number(2), Suit::Spades, State::Deck));
        assert_eq!(true, true);
    }

    #[test]
    fn test_value_addition() {
        assert_eq!(Value::Number(3) + 2, Value::Number(5));
        assert_eq!(Value::Number(10) + 1, Value::Face(FaceType::Jack));
        assert_eq!(Value::Number(10) + 5, Value::Number(2));
        assert_eq!(Value::Number(10) + 4, Value::Ace);
        assert_eq!(Value::Number(10) + (-1), Value::Number(9));
    }

    #[test]
    fn test_value_subtraction() {
        assert_eq!(Value::Number(3) - 1, Value::Number(2));
        assert_eq!(Value::Ace - 4, Value::Number(10));
        assert_eq!(Value::Face(FaceType::King) - 14, Value::Face(FaceType::Queen));
    }

    #[test]
    fn test_value_iterator() {
        for v in Value::values() {
            println!("{:?}", v);
        }
    }
}


