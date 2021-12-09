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
    let mut floor: Vec<Vec<i32>> = Vec::new();
    // Build array of values
    for line in text.lines() {
        floor.push(
            line.chars()
                .map(|c| String::from(c).parse::<i32>().unwrap())
                .collect(),
        );
    }

    let bottom = floor.len();
    let right = floor[0].len();

    let mut sum = 0;
    for (x, row) in floor.iter().enumerate() {
        for (y, val) in row.iter().enumerate() {
            let mut min = true;
            if x != 0 && floor[x - 1][y] <= *val {
                min = false;
            }
            if x < bottom - 1 && floor[x + 1][y] <= *val {
                min = false;
            }
            if y != 0 && floor[x][y - 1] <= *val {
                min = false;
            }
            if y < right - 1 && floor[x][y + 1] <= *val {
                min = false;
            }

            if min {
                sum += val + 1;
            }
        }
    }

    println!("{}", sum);
}

fn part2(text: &str) {}
