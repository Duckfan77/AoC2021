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
    let init = base.chars().collect::<Vec<char>>();
    let mut rules = HashMap::new();

    for line in rulestxt.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        rules.insert(
            (chars[0], chars[1]),
            ((chars[0], chars[6]), (chars[6], chars[1]), chars[6]),
        );
    }

    // Build base so that it contains each pair of the input
    let mut base: HashMap<(char, char), u128> = HashMap::new();

    for pair in init.windows(2) {
        *base.entry((pair[0], pair[1])).or_insert(0) += 1;
    }

    let mut counts = HashMap::new();
    for c in init {
        *counts.entry(c).or_insert(0) += 1;
    }

    for _i in 0..10 {
        //println!("{}", _i);
        base = step(&rules, base, &mut counts);
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    println!("{}", max - min);
}

fn part2(text: &str) {
    let (base, rulestxt) = text.split_once("\n\n").unwrap();
    let init = base.chars().collect::<Vec<char>>();
    let mut rules = HashMap::new();

    for line in rulestxt.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        rules.insert(
            (chars[0], chars[1]),
            ((chars[0], chars[6]), (chars[6], chars[1]), chars[6]),
        );
    }

    // Build base so that it contains each pair of the input
    let mut base: HashMap<(char, char), u128> = HashMap::new();

    for pair in init.windows(2) {
        *base.entry((pair[0], pair[1])).or_insert(0) += 1;
    }

    let mut counts = HashMap::new();
    for c in init {
        *counts.entry(c).or_insert(0) += 1;
    }

    for _i in 0..40 {
        //println!("{}", _i);
        base = step(&rules, base, &mut counts);
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    println!("{}", max - min);
}

fn step(
    rules: &HashMap<(char, char), ((char, char), (char, char), char)>,
    base: HashMap<(char, char), u128>,
    counts: &mut HashMap<char, u128>,
) -> HashMap<(char, char), u128> {
    let mut out = HashMap::new();
    for (key, value) in base.iter() {
        let (p1, p2, c) = rules.get(key).unwrap();
        *out.entry(*p1).or_insert(0) += value;
        *out.entry(*p2).or_insert(0) += value;
        *counts.entry(*c).or_insert(0) += value;
    }

    out
}
