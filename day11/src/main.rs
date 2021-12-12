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
    let mut grid: HashMap<(isize, isize), u8> = HashMap::new();

    for (x, line) in text.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            grid.insert((x as isize, y as isize), String::from(c).parse().unwrap());
        }
    }

    let mut sum = 0;
    for _i in 0..100 {
        //println!("step: {}", _i);
        sum += step(&mut grid);
    }

    println!("{}", sum);
}

fn part2(text: &str) {
    let mut grid: HashMap<(isize, isize), u8> = HashMap::new();

    for (x, line) in text.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            grid.insert((x as isize, y as isize), String::from(c).parse().unwrap());
        }
    }

    let mut count = 0;
    loop {
        count += 1;
        //println!("step: {}", _i);
        if step(&mut grid) == 100 {
            break;
        }
    }

    println!("{}", count);
}

fn step(grid: &mut HashMap<(isize, isize), u8>) -> i32 {
    let mut flash: HashMap<(isize, isize), ()> = HashMap::new();
    let mut to_check = Vec::new();

    for x in 0..10 {
        for y in 0..10 {
            *grid.entry((x, y)).or_insert(0) += 1;
        }
    }

    //Handle all inner points for a first pass
    for x in 0..10 {
        for y in 0..10 {
            if *grid.get(&(x, y)).unwrap() > 9 {
                //println!("Initial Flash at {},{}", x, y);
                flash.insert((x, y), ());
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx != 0 || dy != 0 {
                            match grid.get_mut(&(x + dx, y + dy)) {
                                Some(v) => {
                                    *v += 1;
                                    to_check.push((x + dx, y + dy))
                                }

                                None => (),
                            }
                        }
                    }
                }
            }
        }
    }

    //Check any that got an extra boost from being adjacent to a flash
    while !to_check.is_empty() {
        let (x, y) = to_check.pop().unwrap();

        if *grid.get(&(x, y)).unwrap() > 9 {
            //println!("Chain Flash Found at {},{}", x, y);
            // only keep going if this is a new value being flashed
            if flash.insert((x, y), ()) == None {
                // Get all adjacent
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        // don't get this point
                        if dx != 0 || dy != 0 {
                            // increment adjacent, and add to the list of points to check
                            match grid.get_mut(&(x + dx, y + dy)) {
                                Some(v) => {
                                    *v += 1;
                                    to_check.push((x + dx, y + dy))
                                }

                                None => (),
                            }
                        }
                    }
                }
            }
        }
    }

    for (x, y) in flash.keys() {
        grid.insert((*x, *y), 0);
    }

    //println!("{:?}\n", flash);

    flash.len() as i32
}
