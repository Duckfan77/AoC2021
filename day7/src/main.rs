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

    let mut best_i = *min;
    let mut best_diff = i32::MAX;
    for i in *min..*max {
        let diff = pos.iter().map(|x| i32::abs(x - i)).sum::<i32>();

        if diff < best_diff {
            best_diff = diff;
            best_i = i;
        }
    }

    println!("{}", best_diff);
}

fn part2(text: &str) {
    let pos: Vec<i32> = text.split(',').map(|x| x.parse().unwrap()).collect();

    let min = pos.iter().min().unwrap();
    let max = pos.iter().max().unwrap();

    let mut best_i = *min;
    let mut best_diff = i32::MAX;
    for i in *min..*max {
        let diff = pos
            .iter()
            .map(|x| {
                // This is triangle numbers, use the formula for the nth triangle number
                let n = i32::abs(x - i);
                n * (n + 1) / 2
            })
            .sum::<i32>();

        if diff < best_diff {
            best_diff = diff;
            best_i = i;
        }
    }

    println!("{}", best_diff);
}
