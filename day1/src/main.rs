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
    println!(
        "{}",
        text.lines()
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>()
            .windows(2)
            .filter(|x| x[1] > x[0])
            .count()
    );
}

fn part2(text: &str) {
    println!(
        "{}",
        text.lines()
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>()
            .windows(3)
            .map(|x| x.iter().sum())
            .collect::<Vec<i32>>()
            .windows(2)
            .filter(|x| x[1] > x[0])
            .count()
    );
}
