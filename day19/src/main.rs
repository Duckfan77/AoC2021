use std::collections::HashMap;
use std::collections::HashSet;
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
    let mut abs_points: HashSet<Point3> = HashSet::new();
}

fn part2(text: &str) {}

#[derive(PartialEq, Debug)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

enum Direction {
    Xplus,
    Xminus,
    Yplus,
    Yminus,
    Zplus,
    Zminus,
}

struct Orientation {
    facing: Direction,
    up: Direction,
}

struct Scan {
    id: i32,
    location: Option<Point3>,
    facing: Option<Direction>,
    up: Option<Direction>,
    points: Vec<Point3>,
}

impl Point3 {
    rotate()
}

impl Scan {
    fn points(facing: Direction, up: Direction) {}

    fn intersect(&self, other: &Self) {}

    fn relativePos(facing: Direction, up: Direction) -> HashMap<Point3, Vec<Point3>> {
        HashMap::new()
    }
}
