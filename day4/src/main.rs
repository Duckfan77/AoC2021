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
    let (numbers, mut boards) = parse(text);

    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);

            if board.won() {
                let val = board
                    .rows
                    .iter()
                    .map(|map| {
                        map.iter()
                            .filter_map(|(key, val)| if !*val { Some(key) } else { None })
                            .sum::<i32>()
                    })
                    .sum::<i32>();

                println!("{}", val * number);
                return;
            }
        }
    }
}

fn part2(text: &str) {
    let (numbers, mut boards) = parse(text);

    for number in numbers {
        let mut remove = Vec::new();
        for (index, board) in boards.iter_mut().enumerate() {
            board.mark(number);

            if board.won() {
                remove.push(index);
            }
        }

        if boards.len() == 1 && remove.len() == 1 {
            let val = boards[0]
                .rows
                .iter()
                .map(|map| {
                    map.iter()
                        .filter_map(|(key, val)| if !*val { Some(key) } else { None })
                        .sum::<i32>()
                })
                .sum::<i32>();

            println!("{} {}", number, val * number);
            return;
        }

        for i in remove.iter().rev() {
            boards.remove(*i);
        }

        remove.clear();
    }
}

fn parse(text: &str) -> (Vec<i32>, Vec<Board>) {
    let (first, body) = text.split_once("\n\n").unwrap();

    (
        first.split(',').map(|x| x.parse().unwrap()).collect(),
        body.split("\n\n").map(|x| Board::new(x)).collect(),
    )
}

#[derive(Clone)]
struct Board {
    rows: Vec<HashMap<i32, bool>>,
    cols: Vec<HashMap<i32, bool>>,
}

impl Board {
    fn mark(&mut self, num: i32) {
        for line in self.rows.iter_mut() {
            if line.contains_key(&num) {
                line.insert(num, true);
            }
        }

        for line in self.cols.iter_mut() {
            if line.contains_key(&num) {
                line.insert(num, true);
            }
        }
    }

    fn won(&self) -> bool {
        self.rows
            .iter()
            .chain(self.cols.iter())
            .map(|x| x.values().all(|y| *y))
            .any(|x| x)
    }

    fn new(text: &str) -> Board {
        let rows2 = text
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        let mut rows = Vec::new();

        for row in &rows2 {
            rows.push(
                row.iter()
                    .map(|x| (*x, false))
                    .collect::<HashMap<i32, bool>>(),
            )
        }

        let mut cols = Vec::new();
        for i in 0..5 {
            let mut col = HashMap::new();
            for row in &rows2 {
                col.insert(row[i], false);
            }
            cols.push(col)
        }

        Board { rows, cols }
    }
}
