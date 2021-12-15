use petgraph::algo::dijkstra;
use petgraph::graph::Graph;
use petgraph::prelude::*;
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
    let mut grid: Vec<Vec<(i32, NodeIndex)>> = text
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| (String::from(x).parse().unwrap(), NodeIndex::new(0)))
                .collect()
        })
        .collect();

    //Add all nodes, and store the mappings of grid locations to nodes in the grid array
    let mut g: Graph<(usize, usize), i32> = Graph::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            grid[x][y].1 = g.add_node((x, y));
        }
    }

    let right = grid[0].len() - 1;
    let bottom = grid.len() - 1;

    for (x, row) in grid.iter().enumerate() {
        for (y, (_, node)) in row.iter().enumerate() {
            for neighbor in get_neighbors((x, y), right, bottom) {
                g.add_edge(
                    *node,
                    grid[neighbor.0][neighbor.1].1,
                    grid[neighbor.0][neighbor.1].0,
                );
            }
        }
    }

    let res = dijkstra(&g, grid[0][0].1, Some(grid[bottom][right].1), |e| {
        *e.weight()
    });

    println!("{}", res[&grid[bottom][right].1]);
}

fn part2(text: &str) {
    let smallgrid: Vec<Vec<i32>> = text
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| (String::from(x).parse().unwrap()))
                .collect()
        })
        .collect();

    let right = smallgrid[0].len() - 1;
    let bottom = smallgrid.len() - 1;

    //store locations for nodes in grid 5 times larger in each dimension
    let mut grid: Vec<Vec<(i32, NodeIndex)>> =
        vec![vec![(0, NodeIndex::new(0)); 5 * (bottom + 1)]; 5 * (right + 1)];

    //Add all nodes, and store the mappings of grid locations to nodes in the grid array
    let mut g: Graph<(usize, usize), i32> = Graph::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            grid[x][y].1 = g.add_node((x, y));
            grid[x][y].0 = get_value((x, y), &smallgrid);
        }
    }

    /*
    for row in &grid {
        for (val, _) in row {
            print!("{}", val);
        }
        println!();
    }*/

    let right = grid[0].len() - 1;
    let bottom = grid.len() - 1;

    for (x, row) in grid.iter().enumerate() {
        for (y, (_, node)) in row.iter().enumerate() {
            for neighbor in get_neighbors((x, y), right, bottom) {
                g.add_edge(
                    *node,
                    grid[neighbor.0][neighbor.1].1,
                    grid[neighbor.0][neighbor.1].0,
                );
            }
        }
    }

    let res = dijkstra(&g, grid[0][0].1, Some(grid[bottom][right].1), |e| {
        *e.weight()
    });

    println!("{}", res[&grid[bottom][right].1]);
}

fn get_value(point: (usize, usize), grid: &Vec<Vec<i32>>) -> i32 {
    let b = grid.len();
    let r = grid[0].len();

    let newx = point.0 % b;
    let newy = point.1 % r;

    let adjust = point.0 / b + point.1 / r;

    let out = (grid[newx][newy] + adjust as i32) % 9;

    // make sure you have 9s, not 0s
    if out == 0 {
        9
    } else {
        out
    }
}

fn get_neighbors(point: (usize, usize), right: usize, bottom: usize) -> Vec<(usize, usize)> {
    let x = point.0;
    let y = point.1;

    let mut out = Vec::new();

    if x != 0 {
        out.push((x - 1, y));
    }
    if y != 0 {
        out.push((x, y - 1));
    }
    if x != bottom {
        out.push((x + 1, y));
    }
    if y != right {
        out.push((x, y + 1));
    }

    out
}
