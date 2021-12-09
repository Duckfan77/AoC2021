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

fn part2(text: &str) {
    let mut floor: HashMap<(isize, isize), i32> = HashMap::new();
    // Build map of values
    let bottom: isize = text.lines().collect::<Vec<&str>>().len() as isize;
    let mut right: isize = 0;
    for (x, line) in text.lines().enumerate() {
        right = line.len() as isize;
        for (y, c) in line.chars().enumerate() {
            floor.insert(
                (x as isize, y as isize),
                String::from(c).parse::<i32>().unwrap(),
            );
        }
    }

    // Find Low Point, then find region from that
    // Find region by growing outward until 9 is reached
    // Each point is added to set, and if not already present,
    // added to stack to have neighbors checked and added

    let mut basins: Vec<HashSet<(isize, isize)>> = Vec::new();

    // Find min points
    for x in 0..bottom {
        for y in 0..right {
            let mut min = true;
            let val = floor.get(&(x, y)).unwrap();
            if floor.get(&(x - 1, y)).unwrap_or(&9) <= val {
                min = false;
            }
            if floor.get(&(x + 1, y)).unwrap_or(&9) <= val {
                min = false;
            }
            if floor.get(&(x, y - 1)).unwrap_or(&9) <= val {
                min = false;
            }
            if floor.get(&(x, y + 1)).unwrap_or(&9) <= val {
                min = false;
            }

            // Build basins out of minpoints
            if min {
                let mut basin = HashSet::new();
                let mut points = Vec::new();
                points.push((x, y));
                basin.insert((x, y));

                while !points.is_empty() {
                    let (x, y) = points.pop().unwrap();
                    // Check each point next to the current point
                    if *floor.get(&(x - 1, y)).unwrap_or(&9) != 9 {
                        if basin.insert((x - 1, y)) {
                            points.push((x - 1, y));
                        }
                    }
                    if *floor.get(&(x + 1, y)).unwrap_or(&9) != 9 {
                        if basin.insert((x + 1, y)) {
                            points.push((x + 1, y));
                        }
                    }
                    if *floor.get(&(x, y - 1)).unwrap_or(&9) != 9 {
                        if basin.insert((x, y - 1)) {
                            points.push((x, y - 1));
                        }
                    }
                    if *floor.get(&(x, y + 1)).unwrap_or(&9) != 9 {
                        if basin.insert((x, y + 1)) {
                            points.push((x, y + 1));
                        }
                    }
                }

                basins.push(basin);
            }
        }
    }

    let mut sizes = basins
        .iter()
        .map(|basin| basin.len())
        .collect::<Vec<usize>>();

    sizes.sort_unstable();

    let len = sizes.len();

    println!("{}", sizes[len - 1] * sizes[len - 2] * sizes[len - 3]);
}
