use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file: File = File::open("input").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    text = String::from(text.trim());

    println!("Part 1:");
    part1(&text);
}

fn part1(text: &str) {
    let mut map: Vec<Vec<Space>> = text
        .lines()
        .map(|line| line.chars().map(Space::from_c).collect())
        .collect();

    let size = (map.len(), map[0].len());

    let mut count = 0;

    loop {
        count += 1;
        if !step(&mut map, size) {
            break;
        }
    }

    println!("{}", count);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Down,
    Right,
    Empty,
}

impl Space {
    fn get_adj(&self, p: (usize, usize), size: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Down => ((p.0 + 1) % size.0, p.1),
            Self::Right => (p.0, (p.1 + 1) % size.1),
            Self::Empty => p,
        }
    }

    fn from_c(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'v' => Self::Down,
            '>' => Self::Right,
            _ => panic!("Unexpected character building map: {}", c),
        }
    }
}

fn step(map: &mut Vec<Vec<Space>>, size: (usize, usize)) -> bool {
    let mut to_move = Vec::new();

    //find right to move
    for (x, line) in map.iter().enumerate() {
        for (y, space) in line.iter().enumerate() {
            if *space == Space::Right {
                let next = space.get_adj((x, y), size);
                if map[next.0][next.1] == Space::Empty {
                    to_move.push((x, y));
                }
            }
        }
    }

    //apply moves from to_move
    for (x, y) in &to_move {
        let x = *x;
        let y = *y;
        let next = map[x][y].get_adj((x, y), size);
        map[next.0][next.1] = Space::Right;
        map[x][y] = Space::Empty;
    }

    let moved = !to_move.is_empty();

    to_move.clear();
    //find down to move
    for (x, line) in map.iter().enumerate() {
        for (y, space) in line.iter().enumerate() {
            if *space == Space::Down {
                let next = space.get_adj((x, y), size);
                if map[next.0][next.1] == Space::Empty {
                    to_move.push((x, y));
                }
            }
        }
    }

    //apply moves from to_move
    for (x, y) in &to_move {
        let x = *x;
        let y = *y;
        let next = map[x][y].get_adj((x, y), size);
        map[next.0][next.1] = Space::Down;
        map[x][y] = Space::Empty;
    }

    moved || !to_move.is_empty()
}
