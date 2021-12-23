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
    let mut cubes = HashSet::new();
    for line in text.lines() {
        let mut cub = Cuboid::parse_line(line);
        cub.trim(-50, 50);
        apply_cuboid(&cub, &mut cubes);
    }

    println!("{}", cubes.len());
}

fn part2(text: &str) {
    let mut cubes = Vec::new();
    for (i, line) in text.lines().enumerate() {
        //println!("{}", i);
        let mut cub = Cuboid::parse_line(line);
        cub.trim(-50, 50);
        cubes = cub.non_overlap(cubes);
    }

    println!("{:?}\n{}", cubes, cubes.len());

    println!("{}", cubes.iter().map(|c| c.volume()).sum::<i64>());
}

#[derive(Clone, Debug)]
struct Cuboid {
    on: bool,
    p1: (i32, i32, i32),
    p2: (i32, i32, i32),
}

impl Cuboid {
    fn parse_line(line: &str) -> Cuboid {
        let (key, v) = line.split_once(' ').unwrap();
        let on = key == "on";
        //strip out equals signs and names
        let v = v.replace(|c| c == 'x' || c == 'y' || c == 'z' || c == '=', "");
        let vs = v
            .split(',')
            .flat_map(|p| p.split(".."))
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        Cuboid {
            on,
            p1: (vs[0], vs[2], vs[4]),
            p2: (vs[1], vs[3], vs[5]),
        }
    }

    fn trim(&mut self, min: i32, max: i32) {
        let xlower = i32::max(self.p1.0, min);
        let xupper = i32::min(self.p2.0, max);

        let ylower = i32::max(self.p1.1, min);
        let yupper = i32::min(self.p2.1, max);

        let zlower = i32::max(self.p1.2, min);
        let zupper = i32::min(self.p2.2, max);
        self.p1 = (xlower, ylower, zlower);
        self.p2 = (xupper, yupper, zupper);
    }

    fn splitx(self, x: i32) -> Vec<Self> {
        let mut out = Vec::new();

        if self.p1.0 < x && self.p2.0 > x {
            out.push(Self {
                on: self.on,
                p1: (self.p1.0, self.p1.1, self.p1.2),
                p2: (x, self.p2.1, self.p1.2),
            });
            out.push(Self {
                on: self.on,
                p1: (x, self.p1.1, self.p1.2),
                p2: (self.p2.0, self.p2.1, self.p2.2),
            });
        } else {
            out.push(self)
        }

        //println!("x: {}", out.len());

        out
    }

    fn splity(self, y: i32) -> Vec<Self> {
        let mut out = Vec::new();

        if self.p1.1 < y && self.p2.1 > y {
            out.push(Self {
                on: self.on,
                p1: (self.p1.0, self.p1.1, self.p1.2),
                p2: (self.p2.0, y, self.p1.2),
            });
            out.push(Self {
                on: self.on,
                p1: (self.p1.0, y, self.p1.2),
                p2: (self.p2.0, self.p2.1, self.p2.2),
            });
        } else {
            out.push(self)
        }

        //println!("y: {}", out.len());

        out
    }

    fn splitz(self, z: i32) -> Vec<Self> {
        let mut out = Vec::new();

        if self.p1.2 < z && self.p2.2 > z {
            out.push(Self {
                on: self.on,
                p1: (self.p1.0, self.p1.1, self.p1.2),
                p2: (self.p2.0, self.p2.1, z),
            });
            out.push(Self {
                on: self.on,
                p1: (self.p1.0, self.p1.1, z),
                p2: (self.p2.0, self.p2.1, self.p2.2),
            });
        } else {
            out.push(self)
        }

        //println!("z: {}", out.len());

        out
    }

    fn intersect(&self, other: &Self) -> bool {
        Self::range_intersect((self.p1.0, self.p2.0), (other.p1.0, other.p2.0))
            && Self::range_intersect((self.p1.1, self.p2.1), (other.p1.1, other.p2.1))
            && Self::range_intersect((self.p1.2, self.p2.2), (other.p1.2, other.p2.2))
    }

    fn range_intersect(r1: (i32, i32), r2: (i32, i32)) -> bool {
        let range1 = r1.0..=r1.1;
        let range2 = r2.0..=r2.1;

        range1.contains(&r2.0)
            || range1.contains(&r2.1)
            || range2.contains(&r1.0)
            || range2.contains(&r1.1)
    }

    fn volume(&self) -> i64 {
        (self.p2.0 - self.p1.0) as i64
            * (self.p2.1 - self.p1.1) as i64
            * (self.p2.2 - self.p1.2) as i64
    }

    fn non_overlap(self, mut others: Vec<Self>) -> Vec<Self> {
        let mut new = Vec::new();

        //make new contain the cuboids from others, split so they don't intersect self
        for cub in others.drain(..) {
            if cub.intersect(&self) {
                let split: Vec<Self> = [cub]
                    .into_iter()
                    .flat_map(|c| c.splitx(self.p1.0).into_iter())
                    .flat_map(|c| c.splitx(self.p2.0).into_iter())
                    .flat_map(|c| c.splity(self.p1.1).into_iter())
                    .flat_map(|c| c.splity(self.p2.1).into_iter())
                    .flat_map(|c| c.splitz(self.p1.2).into_iter())
                    .flat_map(|c| c.splitz(self.p2.2).into_iter())
                    .collect();

                new.extend(split.into_iter()); //.filter(|c| !c.intersect(&self)));
            } else {
                new.push(cub);
            }
        }

        //if the new cuboid should be in the set, add it
        if self.on {
            new.push(self);
        }

        new
    }
}

fn apply_cuboid(cub: &Cuboid, map: &mut HashSet<(i32, i32, i32)>) {
    for x in cub.p1.0..=cub.p2.0 {
        for y in cub.p1.1..=cub.p1.1 {
            for z in cub.p1.2..=cub.p2.2 {
                if cub.on {
                    map.insert((x, y, z));
                } else {
                    map.remove(&(x, y, z));
                }
            }
        }
    }
}
