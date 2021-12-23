use euclid::*;
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
    let mut cubes: Vec<Box3D<i64, ()>> = Vec::new();
    for (_i, line) in text.lines().enumerate() {
        println!("{}", _i);
        let mut cub = Cuboid::parse_line(line);
        cub.trim(-50, 50);
        let cuboid = Box3D::new(
            Point3D::new(cub.p1.0, cub.p1.1, cub.p1.2),
            Point3D::new(cub.p2.0, cub.p2.1, cub.p2.2),
        );
        cubes = non_overlap(&cuboid, cubes);
        if cub.on {
            cubes.push(cuboid);
        }
    }

    println!("{}", cubes.len());

    println!("{}", cubes.iter().map(|c| c.volume()).sum::<i64>());
}

#[derive(Clone, Debug)]
struct Cuboid {
    on: bool,
    p1: (i64, i64, i64),
    p2: (i64, i64, i64),
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
            .collect::<Vec<i64>>();

        Cuboid {
            on,
            p1: (vs[0], vs[2], vs[4]),
            p2: (vs[1], vs[3], vs[5]),
        }
    }

    fn trim(&mut self, min: i64, max: i64) {
        let xlower = i64::max(self.p1.0, min);
        let xupper = i64::min(self.p2.0, max);

        let ylower = i64::max(self.p1.1, min);
        let yupper = i64::min(self.p2.1, max);

        let zlower = i64::max(self.p1.2, min);
        let zupper = i64::min(self.p2.2, max);
        self.p1 = (xlower, ylower, zlower);
        self.p2 = (xupper, yupper, zupper);
    }
}

fn apply_cuboid(cub: &Cuboid, map: &mut HashSet<(i64, i64, i64)>) {
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

fn split_cub(cub: &Box3D<i64, ()>, splitee: &Box3D<i64, ()>) -> Vec<Box3D<i64, ()>> {
    let mut out = Vec::new();

    let intersect = match cub.intersection(&splitee) {
        None => {
            // no intersection, no splitting to do
            out.push(*splitee);
            return out;
        }
        Some(i) => i,
    };

    let mut xrange = [Option::<(i64, i64)>::None; 3];
    let mut yrange = [Option::<(i64, i64)>::None; 3];
    let mut zrange = [Option::<(i64, i64)>::None; 3];

    //x
    let min = i64::min(cub.min.x, splitee.min.x);
    let max = i64::max(cub.max.x, splitee.max.x);
    if min != intersect.min.x {
        xrange[0] = Some((min, intersect.min.x));
    }
    xrange[1] = Some((intersect.min.x, intersect.max.x));
    if max != intersect.max.x {
        xrange[2] = Some((intersect.max.x, max));
    }

    //y
    let min = i64::min(cub.min.y, splitee.min.y);
    let max = i64::max(cub.max.y, splitee.max.y);
    if min != intersect.min.y {
        yrange[0] = Some((min, intersect.min.y));
    }
    yrange[1] = Some((intersect.min.y, intersect.max.y));
    if max != intersect.max.y {
        yrange[2] = Some((intersect.max.y, max));
    }

    //z
    let min = i64::min(cub.min.z, splitee.min.z);
    let max = i64::max(cub.max.z, splitee.max.z);
    if min != intersect.min.z {
        zrange[0] = Some((min, intersect.min.z));
    }
    zrange[1] = Some((intersect.min.z, intersect.max.z));
    if max != intersect.max.z {
        zrange[2] = Some((intersect.max.z, max));
    }

    for x in xrange {
        let x = match x {
            None => continue,
            Some(i) => i,
        };
        for y in yrange {
            let y = match y {
                None => continue,
                Some(i) => i,
            };
            for z in zrange {
                let z = match z {
                    None => continue,
                    Some(i) => i,
                };
                out.push(Box3D::new(
                    Point3D::new(x.0, y.0, z.0),
                    Point3D::new(x.1, y.1, z.1),
                ))
            }
        }
    }

    out
}

fn non_overlap(cub: &Box3D<i64, ()>, mut boxes: Vec<Box3D<i64, ()>>) -> Vec<Box3D<i64, ()>> {
    let mut out = Vec::new();

    for other in boxes.drain(..) {
        if cub.intersects(&other) {
            let split = split_cub(cub, &other);
            out.extend(split.into_iter().filter(|c| !c.intersects(cub)));
        } else {
            out.push(other);
        }
    }

    out
}
