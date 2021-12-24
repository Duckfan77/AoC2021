//#![allow(dead_code)]

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

fn part1(_text: &str) {
    let mut states = Vec::new();

    /*
    let start = State::new(
        Room::new(
            vec![Amphipod::Copper, Amphipod::Desert],
            Amphipod::Amber,
            2,
            2,
        ),
        Room::new(
            vec![Amphipod::Desert, Amphipod::Copper],
            Amphipod::Bronze,
            2,
            4,
        ),
        Room::new(
            vec![Amphipod::Amber, Amphipod::Amber],
            Amphipod::Copper,
            2,
            6,
        ),
        Room::new(
            vec![Amphipod::Bronze, Amphipod::Bronze],
            Amphipod::Desert,
            2,
            8,
        ),
    );*/

    let start = State::new(
        Room::new(
            vec![Amphipod::Amber, Amphipod::Bronze],
            Amphipod::Amber,
            2,
            2,
        ),
        Room::new(
            vec![Amphipod::Desert, Amphipod::Copper],
            Amphipod::Bronze,
            2,
            4,
        ),
        Room::new(
            vec![Amphipod::Copper, Amphipod::Bronze],
            Amphipod::Copper,
            2,
            6,
        ),
        Room::new(
            vec![Amphipod::Amber, Amphipod::Desert],
            Amphipod::Desert,
            2,
            8,
        ),
    );

    states.push(start);

    let mut least_cost = i32::MAX;
    while !states.is_empty() {
        let state = states.pop().unwrap();
        //no sense continuing on this branch, more expensive than the best cost found so far
        if state.cost > least_cost {
            continue;
        }

        if state.done() {
            //if the state is in the done state, update the least cost, if it's lower
            if state.cost < least_cost {
                least_cost = state.cost;
            }
        } else {
            //get all the new states, and add them
            states.extend(state.steps());
        }
    }

    println!("{}", least_cost);
}

fn part2(text: &str) {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn cost(&self, dist: i32) -> i32 {
        dist * match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    hallway: Hall,
    rooms: Vec<Room>,
    cost: i32,
}

impl State {
    fn new(room1: Room, room2: Room, room3: Room, room4: Room) -> Self {
        Self {
            hallway: Hall::new(),
            rooms: vec![room1, room2, room3, room4],
            cost: 0,
        }
    }

    fn steps(&self) -> Vec<Self> {
        let mut out = Vec::new();

        //Check all hallway amphipods
        for (i, a) in self.hallway.occupants() {
            for (j, room) in self.rooms.iter().enumerate() {
                if room.goal == a {
                    if room.has_space() && self.hallway.reachable(i).contains(&room.exit) {
                        let mut step = self.clone();
                        let mut dist = step.rooms[j].insert(a).unwrap();
                        dist += Hall::dist(i, room.exit);
                        step.cost += a.cost(dist);
                        out.push(step);
                    }

                    //no other rooms will be valid, stop looking
                    break;
                }
            }
        }

        //Check all in-room amphipods
        for i in 0..self.rooms.len() {
            let mut remstate = self.clone();
            let room = &mut remstate.rooms[i];

            let (rem, mut dist) = match room.remove() {
                Some(r) => r,
                None => continue,
            };

            let exit = room.exit;

            //try to go to own room
            for (j, r) in remstate.rooms.iter().enumerate() {
                if r.goal == rem {
                    if r.has_space() && remstate.hallway.reachable(exit).contains(&r.exit) {
                        let mut step = remstate.clone();
                        dist += step.rooms[j].insert(rem).unwrap();
                        dist += Hall::dist(exit, r.exit);
                        step.cost += rem.cost(dist);

                        out.push(step);
                    }

                    break;
                }
            }

            //Go to a place in the hallway
            let opts = remstate.hallway.reachable(exit);
            for spot in opts {
                if Hall::valid(spot) {
                    let mut step = remstate.clone();
                    step.hallway.spots[spot] = Some(rem);
                    dist += Hall::dist(exit, spot);
                    step.cost += rem.cost(dist);

                    out.push(step);
                }
            }
        }

        out
    }

    fn done(&self) -> bool {
        self.rooms.iter().all(|r| r.done())
    }
}

#[derive(Clone, Debug)]
struct Room {
    exit: usize,
    goal: Amphipod,
    done: usize,
    occupants: Vec<Amphipod>,
    size: usize,
}

impl Room {
    /**
     * occupants is ordered with the bottom cell first
     */
    fn new(occupants: Vec<Amphipod>, goal: Amphipod, size: usize, exit: usize) -> Self {
        let mut done = 0;
        let mut oc = Vec::new();

        for amp in occupants {
            if amp == goal {
                done += 1;
            } else {
                oc.push(amp);
            }
        }

        Self {
            exit,
            goal,
            done,
            occupants: oc,
            size,
        }
    }

    fn has_space(&self) -> bool {
        self.occupants.is_empty() && self.done < self.size
    }

    fn get_goal(&self) -> Amphipod {
        self.goal
    }

    /**
     * Returns None if insertion fails, returns distance of insert if successful
     */
    fn insert(&mut self, amph: Amphipod) -> Option<i32> {
        if !self.has_space() || amph != self.get_goal() {
            None
        } else {
            let dist = self.size - self.done;
            self.done += 1;
            Some(dist as i32)
        }
    }

    fn remove(&mut self) -> Option<(Amphipod, i32)> {
        if self.occupants.is_empty() {
            None
        } else {
            Some((
                self.occupants.pop().unwrap(),
                (self.size - self.occupants.len() + 1) as i32,
            ))
        }
    }

    fn done(&self) -> bool {
        self.done == self.size
    }
}

#[derive(Clone, Debug)]
struct Hall {
    spots: Vec<Option<Amphipod>>,
}

impl Hall {
    fn new() -> Self {
        Self {
            spots: vec![None; 11],
        }
    }

    fn reachable(&self, start: usize) -> Vec<usize> {
        let mut out = Vec::new();

        //go down
        let mut point = start;
        while self.spots[point] == None {
            out.push(point);
            if point == 0 {
                break;
            }
            point -= 1;
        }

        //go up
        point = start + 1;
        while point < self.spots.len() && self.spots[point] == None {
            out.push(point);
            point += 1;
        }

        out
    }

    fn dist(start: usize, end: usize) -> i32 {
        i32::abs(start as i32 - end as i32)
    }

    fn valid(loc: usize) -> bool {
        matches!(loc, 0 | 1 | 3 | 5 | 7 | 9 | 10)
    }

    fn occupants(&self) -> Vec<(usize, Amphipod)> {
        let mut out = Vec::new();

        for (i, a) in self.spots.iter().enumerate() {
            if *a != None {
                out.push((i, a.unwrap()));
            }
        }

        out
    }
}
