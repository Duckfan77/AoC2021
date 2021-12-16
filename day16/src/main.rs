use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file: File = File::open("input").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    text = String::from(text.trim());

    execute(&text);
}

fn execute(text: &str) {
    //convert hex to an array of bool, easier to work with
    let mut input: Vec<bool> = build_vec(text);

    let p = Packet::new(&mut input).unwrap();

    println!("Part1:\n{}", p.sum_version());

    println!("\nPart2:\n{}", p.calc_value());
}

//break-out initial parsing to a function to use in both parts
fn build_vec(text: &str) -> Vec<bool> {
    let mut input: Vec<bool> = Vec::new();
    for c in text.chars() {
        input.extend(match c {
            '0' => [false, false, false, false],
            '1' => [false, false, false, true],
            '2' => [false, false, true, false],
            '3' => [false, false, true, true],
            '4' => [false, true, false, false],
            '5' => [false, true, false, true],
            '6' => [false, true, true, false],
            '7' => [false, true, true, true],
            '8' => [true, false, false, false],
            '9' => [true, false, false, true],
            'A' => [true, false, true, false],
            'B' => [true, false, true, true],
            'C' => [true, true, false, false],
            'D' => [true, true, false, true],
            'E' => [true, true, true, false],
            'F' => [true, true, true, true],
            _ => panic!("Unexpected character {}", c),
        });
    }
    input
}

#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: u64,

    value: u64,

    subpackets: Vec<Packet>,
}

impl Packet {
    fn new(input: &mut Vec<bool>) -> Option<Packet> {
        // must have at least 6 bits, for version and type_id
        if input.len() < 6 {
            return None;
        }

        let mut i;
        let version = slice_to_int(&input[0..3]);
        let type_id = slice_to_int(&input[3..6]);
        i = 6;

        //Handle constant case
        if type_id == 4 {
            let mut value = Vec::new();

            //add each block that starts with a 1
            while input[i] {
                value.extend(&input[i + 1..i + 5]);
                i += 5;
            }
            //one more block once the 0 bit is reached
            value.extend(&input[i + 1..i + 5]);
            i += 5;

            //remove the elements used in this packet
            input.drain(0..i);

            //println!("Parsing int value");
            Some(Packet {
                version,
                type_id,
                value: slice_to_int(&value),
                subpackets: Vec::new(),
            })

        //Handle non-constant case
        } else {
            let length_type = input[6];
            i += 1;

            let mut subpackets = Vec::new();

            //make subpackets array for len-type 1
            if length_type {
                //println!("parsing count for len-type 1");
                let count = slice_to_int(&input[i..i + 11]);
                i += 11;

                //drain used bits, before passing off to make packets
                input.drain(0..i);
                //create count subpackets, each will drain own bits
                for _ in 0..count {
                    let sub = Packet::new(input).unwrap();
                    subpackets.extend([sub]);
                }
            } else {
                //println!("parsing len for len-type 0");
                let len = slice_to_int(&input[i..i + 15]) as usize;
                i += 15;

                let mut sub_array = input[i..i + len].iter().map(|x| *x).collect::<Vec<bool>>();
                i += len;
                while sub_array.len() >= 6 {
                    let sub = Packet::new(&mut sub_array).unwrap();
                    subpackets.extend([sub]);
                }

                //drain used bits, before returning
                input.drain(0..i);
            }

            Some(Packet {
                version,
                type_id,
                value: 0,
                subpackets,
            })
        }
    }

    fn sum_version(&self) -> u64 {
        let mut sum = self.version;

        sum += self.subpackets.iter().map(|p| p.sum_version()).sum::<u64>();

        sum
    }

    fn calc_value(&self) -> u64 {
        match self.type_id {
            0 => self.subpackets.iter().map(|p| p.calc_value()).sum(),
            1 => self.subpackets.iter().map(|p| p.calc_value()).product(),
            2 => self
                .subpackets
                .iter()
                .map(|p| p.calc_value())
                .min()
                .unwrap(),
            3 => self
                .subpackets
                .iter()
                .map(|p| p.calc_value())
                .max()
                .unwrap(),

            4 => self.value,

            5 => {
                if self.subpackets[0].calc_value() > self.subpackets[1].calc_value() {
                    1
                } else {
                    0
                }
            }

            6 => {
                if self.subpackets[0].calc_value() < self.subpackets[1].calc_value() {
                    1
                } else {
                    0
                }
            }

            7 => {
                if self.subpackets[0].calc_value() == self.subpackets[1].calc_value() {
                    1
                } else {
                    0
                }
            }

            _ => panic!("Unexpected type id {}", self.type_id),
        }
    }
}

fn slice_to_str(slice: &[bool]) -> String {
    let mut out = String::new();

    for b in slice {
        if *b {
            out.push('1');
        } else {
            out.push('0');
        }
    }

    out
}

fn slice_to_int(slice: &[bool]) -> u64 {
    //println!("{:?}", slice);
    u64::from_str_radix(&slice_to_str(slice), 2).unwrap()
}
