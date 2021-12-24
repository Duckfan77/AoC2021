use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

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
    let instr = text.lines().map(Op::parse_line).collect::<Vec<Op>>();

    let mut num: i64 = 99_999_999_999_999;

    /*
    for i in 0..=4 {
        let mut arr = vec![4 * i, 4];

        let mut alu = Alu::new();

        for op in &instr {
            op.execute(&mut alu, &mut arr);
        }

        println!(
            "{}: {} {} {} {}",
            i,
            alu.regs.get(&Reg::W).unwrap(),
            alu.regs.get(&Reg::X).unwrap(),
            alu.regs.get(&Reg::Y).unwrap(),
            alu.regs.get(&Reg::Z).unwrap()
        );
    }*/

    let all: Vec<i64> = (0..num)
        .into_par_iter()
        .map(|num| {
            if num % 1_000_000 == 0 {
                println!("num: {}", num);
            }

            let arr = int_to_vec(num);
            (num, arr)
        })
        .filter(|(_, arr)| !arr.contains(&0))
        .map(|(i, mut arr)| {
            let mut alu = Alu::new();

            for op in &instr {
                op.execute(&mut alu, &mut arr);
            }

            (i, alu.done())
        })
        .filter_map(|(i, done)| if done { Some(i) } else { None })
        .collect();

    println!("{:#?}", all);

    println!(
        "min: {}, max: {}, len: {}",
        all.iter().min().unwrap(),
        all.iter().max().unwrap(),
        all.len(),
    );
}

fn part2(text: &str) {}

struct Alu {
    regs: HashMap<Reg, i64>,
}

impl Alu {
    fn new() -> Alu {
        let mut regs = HashMap::new();
        regs.insert(Reg::W, 0);
        regs.insert(Reg::X, 0);
        regs.insert(Reg::Y, 0);
        regs.insert(Reg::Z, 0);

        Alu { regs }
    }

    fn done(&self) -> bool {
        *self.regs.get(&Reg::Z).unwrap_or(&1) == 0
    }
}

enum Inp {
    Reg(Reg),
    Val(i64),
}

impl Inp {
    fn get_val(&self, regs: &Alu) -> i64 {
        match self {
            Inp::Val(i) => *i,
            Inp::Reg(r) => *regs.regs.get(r).unwrap_or(&0),
        }
    }

    fn from_str(input: &str) -> Inp {
        match input.parse::<i64>() {
            Ok(i) => Inp::Val(i),
            Err(_) => Inp::Reg(Reg::from_str(input)),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Reg {
    W,
    X,
    Y,
    Z,
}

impl Reg {
    fn from_str(input: &str) -> Reg {
        match input {
            "w" => Reg::W,
            "x" => Reg::X,
            "y" => Reg::Y,
            "z" => Reg::Z,
            _ => panic!("unexpected reg: {}", input),
        }
    }
}

enum Op {
    Inp(Reg),
    Add(Reg, Inp),
    Mul(Reg, Inp),
    Div(Reg, Inp),
    Mod(Reg, Inp),
    Eql(Reg, Inp),
}

impl Op {
    fn execute(&self, alu: &mut Alu, input: &mut Vec<i64>) {
        match self {
            Op::Inp(r) => {
                alu.regs.insert(*r, input.pop().unwrap());
            }
            Op::Add(r, i) => {
                *alu.regs.entry(*r).or_insert(0) += i.get_val(alu);
            }
            Op::Mul(r, i) => {
                *alu.regs.entry(*r).or_insert(0) *= i.get_val(alu);
            }
            Op::Div(r, i) => {
                *alu.regs.entry(*r).or_insert(0) /= i.get_val(alu);
            }
            Op::Mod(r, i) => {
                *alu.regs.entry(*r).or_insert(0) %= i.get_val(alu);
            }
            Op::Eql(r, i) => {
                if *alu.regs.get(r).unwrap_or(&0) == i.get_val(alu) {
                    alu.regs.insert(*r, 1);
                } else {
                    alu.regs.insert(*r, 0);
                }
            }
        };
    }

    fn parse_line(line: &str) -> Op {
        let args = line.split_whitespace().collect::<Vec<&str>>();
        match args[0] {
            "inp" => Op::Inp(Reg::from_str(args[1])),

            "add" => Op::Add(Reg::from_str(args[1]), Inp::from_str(args[2])),

            "mul" => Op::Mul(Reg::from_str(args[1]), Inp::from_str(args[2])),

            "div" => Op::Div(Reg::from_str(args[1]), Inp::from_str(args[2])),

            "mod" => Op::Mod(Reg::from_str(args[1]), Inp::from_str(args[2])),

            "eql" => Op::Eql(Reg::from_str(args[1]), Inp::from_str(args[2])),

            _ => panic!("Unexpected input {}", args[0]),
        }
    }
}

fn int_to_vec(int: i64) -> Vec<i64> {
    let s = format!("{:0>14}", int);

    s.chars()
        .map(|c| String::from(c).parse::<i64>().unwrap())
        .rev()
        .collect()
}
