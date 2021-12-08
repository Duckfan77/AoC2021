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
    let mut count = 0;
    for line in text.lines() {
        let (_, body) = line.split_once('|').unwrap();
        for num in body.split_whitespace() {
            match num.len() {
                2 | 4 | 3 | 7 => count += 1,

                _ => (),
            }
        }
    }

    println!("{}", count);
}

fn part2(text: &str) {
    println!(
        "{}",
        text.lines().map(|line| handle_line(line)).sum::<i32>()
    )
}

fn make_mappings(body: &str) -> HashMap<char, char> {
    let mut letter_counts: HashMap<char, i32> = HashMap::new();

    for c in body.chars() {
        if c != ' ' {
            *letter_counts.entry(c).or_insert(0) += 1;
        }
    }

    let mut mappings: HashMap<char, char> = HashMap::new();

    let mut one = "";
    let mut four = "";
    let mut seven = "";

    for num in body.split_whitespace() {
        match num.len() {
            2 => one = num,

            4 => four = num,

            3 => seven = num,

            _ => (),
        }
    }

    // Identify segment a, as the only one in 7 not in a
    for ch in seven.chars() {
        if !one.contains(ch) {
            mappings.insert(ch, 'a');
            letter_counts.remove(&ch); // Don't need to look at ch anymore, we know what it is
            break;
        }
    }

    // Identify segments b, c, e, f
    let mut b = ' ';
    let mut seven_count = HashMap::new();
    for (ch, count) in letter_counts.drain() {
        match count {
            6 => {
                b = ch;
                mappings.insert(ch, 'b');
            }

            8 => {
                mappings.insert(ch, 'c');
            }

            4 => {
                mappings.insert(ch, 'e');
            }

            9 => {
                mappings.insert(ch, 'f');
            }

            7 => {
                seven_count.insert(ch, ());
            }

            _ => (),
        }
    }

    // Identify d using b and 4
    for ch in four.chars() {
        if !(one.contains(ch) || ch == b) {
            mappings.insert(ch, 'd');
            seven_count.remove(&ch);
        }
    }

    // g is the other remaining with count 7
    mappings.insert(*seven_count.keys().next().unwrap(), 'g');

    mappings
}

fn handle_line(line: &str) -> i32 {
    let (body, nums) = line.split_once('|').unwrap();

    let mut nums = nums.to_uppercase();

    let mapping = make_mappings(body);

    //println!("{:?}", mapping);

    //remap to correct strings
    for (wire, seg) in mapping {
        nums = nums.replace(wire.to_ascii_uppercase(), &String::from(seg));
    }

    //get four numbers and sort, for easier comparison
    let mut digits: Vec<Vec<char>> = nums
        .split_whitespace()
        .map(|digit| digit.chars().collect())
        .collect();

    //sort and construct output string to parse
    let mut outstring = String::new();

    for digit in digits.iter_mut() {
        digit.sort_unstable();
        let string = digit.iter().cloned().collect::<String>();
        //println!("{}", string);
        match string.as_str() {
            "abcefg" => outstring.push('0'),
            "cf" => outstring.push('1'),
            "acdeg" => outstring.push('2'),
            "acdfg" => outstring.push('3'),
            "bcdf" => outstring.push('4'),
            "abdfg" => outstring.push('5'),
            "abdefg" => outstring.push('6'),
            "acf" => outstring.push('7'),
            "abcdefg" => outstring.push('8'),
            "abcdfg" => outstring.push('9'),

            _ => (),
        }
    }

    outstring.parse().unwrap()
}
