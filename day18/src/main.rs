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
    let mut input: Vec<SnailfishNumber> = text
        .lines()
        .map(|l| SnailfishNumber::from_slice(&l.chars().collect::<Vec<char>>()))
        .collect();
    let mut p = input[0].clone();
    for num in input.drain(1..) {
        //println!("Do Sum");
        p = p + num;
    }

    println!("{}", p.magnitude());
}

fn part2(_text: &str) {}

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
        let p = Self {
            v1: Value::Pair(Box::new(self)),
            v2: Value::Pair(Box::new(other)),
        };
        p.reduce()
    }
}

impl SnailfishNumber {
    fn reduce(self) -> Self {
        let mut p = self.clone();

        loop {
            //explode until you can't anymore
            let mut v = Self::to_str(p);
            while Self::explode_str(&mut v) {}

            /*
            println!("Convert back after exploding");
            for c in &v {
                print!("{}", c);
            }
            println!();*/

            //convert back to pair notation to split
            p = Self::from_slice(&v);
            //done exploding, and can't split, reduction complete
            if !p.split() {
                break;
            }
        }

        p
    }

    fn explode_str(input: &mut Vec<char>) -> bool {
        let mut stack = Vec::new();
        let mut do_explode = false;
        let mut pair_i = 0;

        /*
        for c in input.iter() {
            print!("{}", c);
        }
        println!();*/

        for (i, c) in input[..].iter().enumerate() {
            match c {
                '[' => {
                    //do explode, now that we found the start of the point to do so on.
                    if stack.len() == 4 {
                        pair_i = i;
                        do_explode = true;
                        break;
                    } else {
                        stack.push(i);
                    }
                }

                ']' => {
                    stack.pop();
                }

                _ => (),
            };
        }

        if do_explode {
            let mut iend = pair_i + 1;
            while input[iend] != ']' {
                iend += 1;
            }
            //println!("Making pair to explode: {:?} {} {}", input, pair_i, iend);
            let p = Self::from_slice(&input[pair_i..=iend]);

            //Handle right, then current location, then left, to avoid modifying
            //indices as values move

            //find locatin of next number
            let mut righti = iend;
            while !input[righti].is_digit(10) {
                righti += 1;
                if righti == input.len() - 1 {
                    break;
                }
            }

            //add to right
            if righti != input.len() - 1 {
                let mut rightend = righti + 1;
                while input[rightend].is_digit(10) {
                    rightend += 1;
                }
                let right: i32 = input[righti..rightend]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                let newrightv = right
                    + match p.v2 {
                        Value::Val(v) => v,
                        Value::Pair(_) => panic!("exploded pair with pair left side"),
                    };

                input.splice(righti..rightend, newrightv.to_string().chars());
            }

            //Replace current pair with a '0'
            input[pair_i] = '0';
            input.drain(pair_i + 1..=iend);

            //find location of previous number
            let mut lefti = pair_i - 1;
            while !input[lefti].is_digit(19) {
                lefti -= 1;
                if lefti == 0 {
                    break;
                }
            }

            //lefti exists
            if lefti != 0 {
                let mut leftend = lefti - 1;
                while input[leftend].is_digit(10) {
                    leftend -= 1;
                }
                let left: i32 = input[leftend + 1..=lefti]
                    .iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                let newleftv = left
                    + match p.v1 {
                        Value::Val(v) => v,
                        Value::Pair(_) => panic!("exploded pair with pair left side"),
                    };

                input.splice(leftend + 1..=lefti, newleftv.to_string().chars());
            }
        }

        do_explode
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
        let mut s2: &[char] = &['0'; 0];
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

        //println!("{:?} | {:?} | {:?}", input, s1, s2);
        let v1 = if s1[0].is_digit(10) {
            Value::Val(i32::from_str_radix(&s1.iter().collect::<String>(), 10).unwrap())
        } else {
            Value::Pair(Box::new(Self::from_slice(s1)))
        };

        let v2 = if s2[0].is_digit(10) {
            Value::Val(i32::from_str_radix(&s2.iter().collect::<String>(), 10).unwrap())
        } else {
            Value::Pair(Box::new(Self::from_slice(s2)))
        };

        Self { v1, v2 }
    }

    fn to_str(input: Self) -> Vec<char> {
        let mut out = Vec::new();

        out.push('[');
        match input.v1 {
            Value::Val(i) => out.extend_from_slice(&i.to_string().chars().collect::<Vec<char>>()),
            Value::Pair(b) => out.append(&mut Self::to_str(*b)),
        }
        out.push(',');
        match input.v2 {
            Value::Val(i) => out.extend_from_slice(&i.to_string().chars().collect::<Vec<char>>()),
            Value::Pair(b) => out.append(&mut Self::to_str(*b)),
        }
        out.push(']');

        out
    }
}
