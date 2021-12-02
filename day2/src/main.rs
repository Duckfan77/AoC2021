use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file: File = File::open("input").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

fn part1(text: &str) {
    let mut h = 0;
    let mut d = 0;

    for line in text.lines() {
        let (key, val) = line.split_once(' ').unwrap();
        let val = val.parse::<i32>().unwrap();

        match key {
            "forward" => h += val,

            "up" => d -= val,

            "down" => d += val,

            _ => (),
        };
    }

    println!("{}", h * d);
}

fn part2(text: &str) {
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;

    for line in text.lines() {
        let (key, val) = line.split_once(' ').unwrap();
        let val = val.parse::<i32>().unwrap();

        match key {
            "forward" => {
                h += val;
                d += aim * val;
            }

            "up" => aim -= val,

            "down" => aim += val,

            _ => (),
        };
    }

    println!("{}", h * d);
}
