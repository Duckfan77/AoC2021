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
    let (base, rulestxt) = text.split_once("\n\n").unwrap();
    let mut base = String::from(base);
    let mut rules = HashMap::new();

    for line in rulestxt.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        rules.insert((chars[0], chars[1]), chars[6]);
    }

    for _ in 0..10 {
        base = step(&rules, base);
    }

    let mut counts = HashMap::new();

    for c in base.chars() {
        *counts.entry(c).or_insert(0) += 1
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    println!("{}", max - min);
}

fn part2(text: &str) {
    let (base, rulestxt) = text.split_once("\n\n").unwrap();
    let mut base = String::from(base);
    let mut rules = HashMap::new();

    for line in rulestxt.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        rules.insert((chars[0], chars[1]), chars[6]);
    }

    for _i in 0..40 {
        println!("{}", _i);
        base = step(&rules, base);
    }

    let mut counts = HashMap::new();

    for c in base.chars() {
        *counts.entry(c).or_insert(0) += 1
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    println!("{}", max - min);
}

fn step(rules: &HashMap<(char, char), char>, mut base: String) -> String {
    let mut chars = base.drain(..);
    let mut out = String::new();

    let mut c1 = chars.next().unwrap();
    for c2 in chars {
        match rules.get(&(c1, c2)) {
            Some(c) => {
                out.push(c1);
                out.push(*c)
            }

            None => out.push(c1),
        }
        c1 = c2;
    }

    out.push(c1);

    out
}
