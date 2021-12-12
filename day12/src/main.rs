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
    let map = build_map(text);

    // store the current node
    let node = String::from("start");

    // store the visited small caves
    let visited: HashMap<String, ()> = HashMap::new();

    println!("{}", traverse(&map, node, visited));
}

fn part2(text: &str) {
    let map = build_map(text);

    // store the current node
    let node = String::from("start");

    // store the visited small caves
    let visited: HashMap<String, i32> = HashMap::new();

    println!("{}", traverse2(&map, node, visited, false));
}

struct Node {
    name: String,
    is_big: bool,
    neighbors: Vec<String>,
}

impl Node {
    fn new(name: String) -> Node {
        let is_big = name.chars().next().unwrap().is_ascii_uppercase();

        Node {
            name,
            is_big,
            neighbors: Vec::new(),
        }
    }
}

fn build_map(text: &str) -> HashMap<String, Node> {
    let mut map = HashMap::new();

    for line in text.lines() {
        let (n1, n2) = line.split_once('-').unwrap();

        // Add n2 to n1's adjacency list, creating n1 if needed
        map.entry(String::from(n1))
            .or_insert(Node::new(String::from(n1)))
            .neighbors
            .push(String::from(n2));

        // Add n1 to n2's adjaceny list, creating n2 if needed
        map.entry(String::from(n2))
            .or_insert(Node::new(String::from(n2)))
            .neighbors
            .push(String::from(n1));
    }

    map
}

/**
 * Takes in the map, the current node, and the visited nodes, and returns how many paths
 * there are to the end from the current node to the end that visit at least one small
 * cave, but visit no small cave more than once
 */
fn traverse(map: &HashMap<String, Node>, node: String, mut visited: HashMap<String, ()>) -> i32 {
    // made it to the end, check if the path taken includes a small cave
    if node == "end" {
        // will always have start, which counts as a small cave, need 1 more
        if visited.len() > 1 {
            return 1;
        } else {
            return 0;
        }
    }

    let n = map.get(&node).unwrap();

    if !n.is_big {
        visited.insert(n.name.clone(), ());
    }

    let mut count = 0;

    for nbr in &n.neighbors {
        if !visited.contains_key(nbr) {
            count += traverse(map, String::from(nbr), visited.clone());
        }
    }

    count
}

/**
 * Takes in the map, the current node, and the visited nodes, and returns how many paths
 * there are to the end from the current node to the end that visit at least one small
 * cave, but visit no small cave more than once
 */
fn traverse2(
    map: &HashMap<String, Node>,
    node: String,
    mut visited: HashMap<String, i32>,
    mut doubled: bool,
) -> i32 {
    // made it to the end, check if the path taken includes a small cave
    if node == "end" {
        // will always have start, which counts as a small cave, need 1 more
        return 1;
    }

    let n = map.get(&node).unwrap();

    // increment number of times visited
    if !n.is_big {
        *visited.entry(node.clone()).or_insert(0) += 1;
        doubled |= *visited.get(&node).unwrap() == 2;
    }

    //Special case, start is counted as visited twice from the start, so it isn't revisited
    if node == "start" {
        println!("Visiting Start");
        *visited.get_mut(&node).unwrap() = 2;
    }

    let mut count = 0;

    for nbr in &n.neighbors {
        //can traverse to a visited cave if doubling hasn't happened yet
        if (!doubled && *visited.get(nbr).unwrap_or(&0) < 2) || !visited.contains_key(nbr) {
            count += traverse2(map, String::from(nbr), visited.clone(), doubled);
        }
    }

    count
}
