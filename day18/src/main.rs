use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

fn main() {
    let mut file: File = File::open("input").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    text = String::from(text.trim());

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

fn part1(text: &str) {
    let p = SnailfishNumber::from_slice(&text.chars().collect::<Vec<char>>());
    println!("{:?}", p);
}

fn part2(text: &str) {}

#[derive(Debug, Clone)]
struct SnailfishNumber {
    v1: Value,
    v2: Value,
}

#[derive(Debug, Clone)]
enum Value {
    Val(i32),

    Pair(Box<SnailfishNumber>),
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            v1: Value::Pair(Box::new(self)),
            v2: Value::Pair(Box::new(other)),
        }
    }
}

impl SnailfishNumber {
    fn reduce(self) -> Self {
        let mut p = self.clone();

        loop {
            //explode until you can't anymore
            while p.explode() {}

            //done exploding, and can't split, reduction complete
            if !p.split() {
                break;
            }
        }

        p
    }

    fn explode(&mut self) -> bool {
        let mut left: Option<&i32> = None;
        let mut right: Option<&i32> = None;

        let mut depth = 0;

        let mut stack: Vec<&mut Self> = Vec::new();
        stack.push(self);

        while !stack.is_empty() {
            let pointer = stack.pop();
            depth += 1;
        }

        true
    }

    fn split(&mut self) -> bool {
        let mut stack: Vec<&mut Value> = Vec::new();

        stack.push(&mut self.v2);
        stack.push(&mut self.v1);

        while !stack.is_empty() {
            let v = stack.pop().unwrap();
            match v {
                Value::Val(i) => {
                    if *i >= 10 {
                        *v = Value::Pair(Box::new(SnailfishNumber {
                            v1: Value::Val(*i / 2),
                            v2: Value::Val(if *i % 2 == 0 { *i / 2 } else { *i / 2 + 1 }),
                        }));
                        return true;
                    }
                }

                Value::Pair(ref mut b) => {
                    stack.push(&mut b.v2);
                    stack.push(&mut b.v1);
                }
            }
        }

        false
    }

    fn find_first_i(&mut self) -> &i32 {
        let mut pointer = self;
        loop {
            match pointer.v1 {
                Value::Val(ref mut i) => return i,

                Value::Pair(ref mut b) => pointer = b,
            }
        }
    }

    fn magnitude(&self) -> i32 {
        let left = match &self.v1 {
            Value::Val(i) => *i,

            Value::Pair(p) => p.magnitude(),
        };

        let right = match &self.v2 {
            Value::Val(i) => *i,
            Value::Pair(p) => p.magnitude(),
        };

        3 * left + 2 * right
    }

    fn from_slice(input: &[char]) -> Self {
        let mut s1 = input;
        let mut s2 = input;
        let mut stack = Vec::new();
        for (i, c) in input[1..].iter().enumerate() {
            match c {
                '[' => {
                    stack.push(i);
                }

                ']' => {
                    stack.pop();
                }

                ',' => {
                    //delimeter between two sides of pair
                    if stack.is_empty() {
                        //use i + 1, because offset 1 from the start of the string
                        s1 = &input[1..i + 1];
                        //start right after this, and don't include trailing ] used to close pair
                        s2 = &input[i + 2..input.len() - 1];
                        break;
                    }
                }

                _ => (),
            };
        }

        let v1 = if s1[0].is_digit(10) {
            Value::Val(s1.iter().collect::<String>().parse().unwrap())
        } else {
            Value::Pair(Box::new(Self::from_slice(s1)))
        };

        let v2 = if s2[0].is_digit(10) {
            Value::Val(s2.iter().collect::<String>().parse().unwrap())
        } else {
            Value::Pair(Box::new(Self::from_slice(s2)))
        };

        Self { v1, v2 }
    }
}
