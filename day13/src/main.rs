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
    let (points, folds) = text.split_once("\n\n").unwrap();
    let mut paper = points
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect::<HashSet<(i32, i32)>>();

    for line in folds.lines() {
        let (body, fold_line) = line.split_once('=').unwrap();
        let fold_line = fold_line.parse::<i32>().unwrap();
        if body == "fold along x" {
            foldx(&mut paper, fold_line);
        } else {
            foldy(&mut paper, fold_line);
        }
        break;
    }

    println!("{}", paper.len());
}

fn part2(text: &str) {
    let (points, folds) = text.split_once("\n\n").unwrap();
    let mut paper = points
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect::<HashSet<(i32, i32)>>();

    for line in folds.lines() {
        let (body, fold_line) = line.split_once('=').unwrap();
        let fold_line = fold_line.parse::<i32>().unwrap();
        if body == "fold along x" {
            foldx(&mut paper, fold_line);
        } else {
            foldy(&mut paper, fold_line);
        }
    }

    for y in 0..6 {
        for x in 0..40 {
            if paper.contains(&(x, y)) {
                print!("x");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn foldx(paper: &mut HashSet<(i32, i32)>, fold: i32) {
    // get a list of all folded away
    let folded = paper
        .iter()
        .filter(|(x, _)| *x > fold)
        .map(|pair| *pair)
        .collect::<Vec<(i32, i32)>>();

    // Remove the ones that are being folded away
    paper.retain(|(x, _)| *x < fold);

    // Re-add the folded ones, offset to match the new location.
    for (x, y) in folded {
        // place the new point as many spots before fold, as fold was before the original x
        paper.insert((fold - (x - fold), y));
    }
}

fn foldy(paper: &mut HashSet<(i32, i32)>, fold: i32) {
    // get a list of all folded away
    let folded = paper
        .iter()
        .filter(|(_, y)| *y > fold)
        .map(|pair| *pair)
        .collect::<Vec<(i32, i32)>>();

    // Remove the ones that are being folded away
    paper.retain(|(_, y)| *y < fold);

    // Re-add the folded ones, offset to match the new location.
    for (x, y) in folded {
        // place the new point as many spots before fold, as fold was before the original x
        paper.insert((x, fold - (y - fold)));
    }
}
