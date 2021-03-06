use std::collections::HashMap;
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
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();
    for line in text.lines() {
        let vents = Line::new(line);
        if vents.straight() {
            for point in vents.points() {
                *points.entry(point).or_insert(0) += 1;
            }
        }
    }

    println!("{}", points.values().filter(|x| **x > 1).count())
}

fn part2(text: &str) {
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();
    for line in text.lines() {
        let vents = Line::new(line);
        for point in vents.points() {
            *points.entry(point).or_insert(0) += 1;
        }
    }

    println!("{}", points.values().filter(|x| **x > 1).count())
}

struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Line {
    fn new(line: &str) -> Line {
        let (p1, p2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(',').unwrap();
        let (x2, y2) = p2.split_once(',').unwrap();

        let x1 = x1.parse::<i32>().unwrap();
        let x2 = x2.parse::<i32>().unwrap();
        let y1 = y1.parse::<i32>().unwrap();
        let y2 = y2.parse::<i32>().unwrap();

        Line { x1, x2, y1, y2 }
    }

    fn horiz(&self) -> bool {
        self.x1 == self.x2
    }

    fn vert(&self) -> bool {
        self.y1 == self.y2
    }

    fn straight(&self) -> bool {
        self.horiz() || self.vert()
    }

    fn points(&self) -> Vec<(i32, i32)> {
        let mut points = Vec::new();

        let yadjust = if self.y1 == self.y2 {
            0
        } else if self.y1 < self.y2 {
            1
        } else {
            -1
        };
        let xadjust = if self.x1 == self.x2 {
            0
        } else if self.x1 < self.x2 {
            1
        } else {
            -1
        };
        let mut x = self.x1;
        let mut y = self.y1;

        //Go until both are at the end point
        while x != self.x2 || y != self.y2 {
            points.push((x, y));
            x += xadjust;
            y += yadjust;
        }
        points.push((x, y));

        points
    }
}
