use std::collections::VecDeque;
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
    let mut fish = VecDeque::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    for i in text.split(',') {
        let i = i.parse().unwrap();
        fish[i] += 1;
    }

    for _ in 0..80 {
        step(&mut fish);
    }

    println!("{}", fish.iter().map(|x| *x as i128).sum::<i128>())
}

fn part2(text: &str) {
    let mut fish = VecDeque::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    for i in text.split(',') {
        let i = i.parse().unwrap();
        fish[i] += 1;
    }

    for _ in 0..256 {
        step(&mut fish);
    }

    println!("{}", fish.iter().map(|x| *x as i128).sum::<i128>())
}

fn step(fish: &mut VecDeque<i64>) {
    let new = fish.pop_front().unwrap();
    fish[6] += new;
    fish.push_back(new);
}

fn _step_old(fish: &mut Vec<i32>) {
    let mut new = Vec::new();

    for i in fish.iter_mut() {
        if *i == 0 {
            *i = 6;
            new.push(8);
        } else {
            *i -= 1;
        }
    }

    fish.append(&mut new);
}

fn _part1_old(text: &str) {
    let mut fish = text.split(',').map(|x| x.parse().unwrap()).collect();

    for _ in 0..80 {
        _step_old(&mut fish);
    }

    println!("{}", fish.len())
}
