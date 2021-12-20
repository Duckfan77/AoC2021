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
    let (alg, imgtxt) = text.split_once("\n\n").unwrap();
    let alg: Vec<bool> = alg.chars().map(|c| c == '#').collect();

    let mut low: isize = 0;
    let mut high: isize = 0;

    let mut img: HashMap<(isize, isize), bool> = HashMap::new();
    for (i, line) in imgtxt.lines().enumerate() {
        high = line.len() as isize;
        for (j, c) in line.chars().enumerate() {
            img.insert((i as isize, j as isize), c == '#');
        }
    }

    //high += 50;

    let mut outside = false;
    //print_img(&img, low - 1, high + 1, outside);
    //println!("{}\n", img.values().filter(|x| **x).count());

    for _ in 0..2 {
        //low-1 and ..=high because we need to go one beyond the bounds of the img to get new pixels
        let mut img2 = HashMap::new();
        for i in low - 1..=high {
            for j in low - 1..=high {
                img2.insert((i, j), alg[getindex(&img, (i, j), outside)]);
            }
        }

        img = img2;
        low -= 1;
        high += 1;
        outside = alg[slice_to_int(&[outside; 9]) as usize];

        //print_img(&img, low, high, outside);
        //println!("{}, {}\n", img.values().filter(|x| **x).count(), outside);
    }

    println!("{}", img.values().filter(|x| **x).count());
}

fn part2(text: &str) {
    let (alg, imgtxt) = text.split_once("\n\n").unwrap();
    let alg: Vec<bool> = alg.chars().map(|c| c == '#').collect();

    let mut low: isize = 0;
    let mut high: isize = 0;

    let mut img: HashMap<(isize, isize), bool> = HashMap::new();
    for (i, line) in imgtxt.lines().enumerate() {
        high = line.len() as isize;
        for (j, c) in line.chars().enumerate() {
            img.insert((i as isize, j as isize), c == '#');
        }
    }

    //high += 50;

    let mut outside = false;
    //print_img(&img, low - 1, high + 1, outside);
    //println!("{}\n", img.values().filter(|x| **x).count());

    for _ in 0..50 {
        //low-1 and ..=high because we need to go one beyond the bounds of the img to get new pixels
        let mut img2 = HashMap::new();
        for i in low - 1..=high {
            for j in low - 1..=high {
                img2.insert((i, j), alg[getindex(&img, (i, j), outside)]);
            }
        }

        img = img2;
        low -= 1;
        high += 1;
        outside = alg[slice_to_int(&[outside; 9]) as usize];

        //print_img(&img, low, high, outside);
        //println!("{}, {}\n", img.values().filter(|x| **x).count(), outside);
    }

    println!("{}", img.values().filter(|x| **x).count());
}

fn getindex(img: &HashMap<(isize, isize), bool>, point: (isize, isize), outside: bool) -> usize {
    let mut idx: Vec<bool> = Vec::new();
    for di in -1..=1 {
        for dj in -1..=1 {
            idx.push(*img.get(&(point.0 + di, point.1 + dj)).unwrap_or(&outside));
        }
    }

    let out = slice_to_int(&idx) as usize;

    //println!("{:?}, {}", point, out);
    out
}

fn slice_to_str(slice: &[bool]) -> String {
    let mut out = String::new();

    for b in slice {
        if *b {
            out.push('1');
        } else {
            out.push('0');
        }
    }

    out
}

fn slice_to_int(slice: &[bool]) -> u64 {
    //println!("{:?}", slice);
    u64::from_str_radix(&slice_to_str(slice), 2).unwrap()
}

#[allow(dead_code)]
fn print_img(img: &HashMap<(isize, isize), bool>, low: isize, high: isize, outside: bool) {
    for i in low..high {
        for j in low..high {
            print!(
                "{}",
                if *img.get(&(i, j)).unwrap_or(&outside) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}
