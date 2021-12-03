use std::collections::HashSet;
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
    let mut bits = vec![0; 12];

    for line in text.lines() {
        for (i, c) in line.chars().enumerate() {
            bits[i] += match c {
                '1' => 1,

                '0' => -1,

                _ => 0,
            }
        }
    }

    println!("{:?}", bits.iter().map(|x| *x > 0).collect::<Vec<bool>>());
}

fn part2(text: &str) {
    let lines = text
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let mut ox = HashSet::new();
    let mut co2 = HashSet::new();

    for i in 0..lines.len() {
        ox.insert(i);
        co2.insert(i);
    }

    let mut idx = 0;

    while ox.len() != 1 {
        let ones = ox
            .iter()
            .map(|spot| if lines[*spot][idx] { 1 } else { -1 })
            .sum::<i32>()
            >= 0;

        ox.retain(|x| lines[*x][idx] == ones);
        idx += 1;
    }

    idx = 0;

    while co2.len() != 1 {
        let ones = co2
            .iter()
            .map(|spot| if lines[*spot][idx] { 1 } else { -1 })
            .sum::<i32>()
            >= 0;

        co2.retain(|x| lines[*x][idx] != ones);
        idx += 1;
    }

    println!(
        "{:?}\n{:?}",
        lines[*co2.iter().next().unwrap()],
        lines[*ox.iter().next().unwrap()]
    )
}
