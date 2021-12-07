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
    let pos: Vec<i32> = text.split(',').map(|x| x.parse().unwrap()).collect();

    let min = pos.iter().min().unwrap();
    let max = pos.iter().max().unwrap();

    let best_diff = (*min..*max)
        .into_iter()
        .map(|i| pos.iter().map(|x| i32::abs(*x - i)).sum::<i32>())
        .min()
        .unwrap();

    println!("{}", best_diff);
}

fn part2(text: &str) {
    let pos: Vec<i32> = text.split(',').map(|x| x.parse().unwrap()).collect();

    let min = pos.iter().min().unwrap();
    let max = pos.iter().max().unwrap();

    let best_diff = (*min..*max)
        .into_iter()
        .map(|i| {
            pos.iter()
                .map(|x| {
                    let n = i32::abs(*x - i);
                    n * (n + 1) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap();

    println!("{}", best_diff);
}
