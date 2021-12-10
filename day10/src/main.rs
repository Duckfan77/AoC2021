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
    let mut sum = 0;

    let mut stack = Vec::new();
    for line in text.lines() {
        let chrs = line.chars().collect::<Vec<char>>();
        let mut err = 0;
        for c in chrs {
            match c {
                '(' => stack.push(Open::Paren),
                '[' => stack.push(Open::Bracket),
                '{' => stack.push(Open::Brace),
                '<' => stack.push(Open::Angle),

                ')' => {
                    if stack.pop().unwrap() != Open::Paren {
                        err = 3;
                        break;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != Open::Bracket {
                        err = 57;
                        break;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != Open::Brace {
                        err = 1197;
                        break;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != Open::Angle {
                        err = 25137;
                        break;
                    }
                }

                _ => panic!("Unexpected character {}", c),
            }
        }

        //println!("Error found {}", err);
        sum += err;
    }

    println!("{}", sum);
}

fn part2(text: &str) {
    let mut scores: Vec<i64> = Vec::new();

    for (i, line) in text.lines().enumerate() {
        let chrs = line.chars().collect::<Vec<char>>();
        let mut stack = Vec::new();
        let mut err = false;
        for c in chrs {
            match c {
                '(' => stack.push(Open::Paren),
                '[' => stack.push(Open::Bracket),
                '{' => stack.push(Open::Brace),
                '<' => stack.push(Open::Angle),

                ')' => {
                    if stack.pop().unwrap() != Open::Paren {
                        err = true;
                        break;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != Open::Bracket {
                        err = true;
                        break;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != Open::Brace {
                        err = true;
                        break;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != Open::Angle {
                        err = true;
                        break;
                    }
                }

                _ => panic!("Unexpected character {}", c),
            }
        }

        if !err {
            let mut score = 0;
            //println!("{}: {:?}", i, stack);
            while !stack.is_empty() {
                score *= 5;
                score += match stack.pop().unwrap() {
                    Open::Paren => 1,
                    Open::Bracket => 2,
                    Open::Brace => 3,
                    Open::Angle => 4,
                }
            }
            //println!("{}", score);
            scores.push(score);
        }
    }

    scores.sort_unstable();
    //println!("{:?}", scores);
    println!("{}", scores[scores.len() / 2]);
}

#[derive(PartialEq, Debug)]
enum Open {
    Paren,
    Bracket,
    Brace,
    Angle,
}
