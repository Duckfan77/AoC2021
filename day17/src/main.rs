fn main() {
    //just hardcode the goals

    let goalx = (57, 116);
    let goaly = (-198, -148);

    /*
    let goalx = (20, 30);
    let goaly = (-10, -5);*/

    println!("Part 1:");
    part1(goalx, goaly);

    println!("\nPart 2:");
    part2(goalx, goaly);
}

fn part1(goalx: (i32, i32), goaly: (i32, i32)) {
    //use 11 for dx, is a triangular number that fits
    let dx = 11;

    let mut maxy = 0;
    println!("Seeking max y");
    //dy range tuned until the value stopped increasing.
    for dy in 0..1000 {
        //println!("{}", dy);
        let mut p = Probe::new(dx, dy, goalx, goaly);
        let mut besty = 0;

        let mut state = State::Running;
        while state == State::Running {
            state = p.step();
            if besty < p.y {
                besty = p.y;
            }
        }

        //println!("{} {}", p.x, p.y);
        if state == State::Success {
            if maxy < besty {
                maxy = besty;
            }
        } else {
            //break;
        }
    }

    println!("{}", maxy);
}

fn part2(goalx: (i32, i32), goaly: (i32, i32)) {
    let mut count = 0;
    //dx and dy ranges tuned until the value stopped increasing.
    for dx in 0..1000 {
        for dy in -500..1000 {
            //println!("{}", dy);
            let mut p = Probe::new(dx, dy, goalx, goaly);

            let mut state = State::Running;
            while state == State::Running {
                state = p.step();
            }

            //println!("{} {}", p.x, p.y);
            if state == State::Success {
                count += 1;
            } else {
                //break;
            }
        }
    }

    println!("{}", count);
}

struct Probe {
    dx: i32,
    dy: i32,
    x: i32,
    y: i32,
    goalx: (i32, i32),
    goaly: (i32, i32),
}

#[derive(PartialEq)]
enum State {
    Running,
    Success,
    Dead,
}

impl Probe {
    fn new(dx: i32, dy: i32, goalx: (i32, i32), goaly: (i32, i32)) -> Probe {
        Probe {
            dx,
            dy,
            x: 0,
            y: 0,
            goalx,
            goaly,
        }
    }

    fn success(&self) -> bool {
        self.goalx.0 <= self.x
            && self.goalx.1 >= self.x
            && self.goaly.0 <= self.y
            && self.goaly.1 >= self.y
    }

    fn dead(&self) -> bool {
        //println!("{} {}", self.goalx.1 < self.x, self.goaly.0 > self.y);
        self.goalx.1 < self.x || self.goaly.0 > self.y
    }

    fn step(&mut self) -> State {
        self.x += self.dx;
        self.y += self.dy;

        self.dx += if self.dx < 0 {
            1
        } else if self.dx > 0 {
            -1
        } else {
            0
        };

        self.dy -= 1;

        if self.success() {
            State::Success
        } else if self.dead() {
            State::Dead
        } else {
            State::Running
        }
    }
}
