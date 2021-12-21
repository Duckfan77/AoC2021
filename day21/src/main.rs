use std::collections::HashMap;

fn main() {
    println!("Part 1:");
    part1();

    println!("\nPart 2:");
    part2();
}

fn part1() {
    let die = Box::new(Deterministic::new());
    let mut game = Game::new(4, 10, die, 1000);

    loop {
        if game.turn1() {
            println!("{}", game.p2.score * game.rolls);
            break;
        }
        if game.turn2() {
            println!("{}", game.p1.score * game.rolls);
            break;
        }
    }
}

fn part2() {
    let mut game = Game2::new(4, 10, 21);

    while !game.games.is_empty() {
        game.step(true);
        game.step(false);
    }

    if game.p1won > game.p2won {
        println!("{}", game.p1won);
    } else {
        println!("{}", game.p2won);
    }
}

//part 1 infrastructure
trait Die {
    fn roll(&mut self) -> i32;
}

struct Deterministic {
    count: i32,
}

impl Deterministic {
    fn new() -> Self {
        Self { count: 1 }
    }
}

impl Die for Deterministic {
    fn roll(&mut self) -> i32 {
        let out = self.count;
        self.count += 1;

        if self.count > 100 {
            self.count = 1;
        }

        out
    }
}

struct Player {
    place: i32,
    score: i32,
}

struct Game {
    p1: Player,
    p2: Player,
    rolls: i32,
    die: Box<dyn Die>,
    win: i32,
}

impl Game {
    fn new(p1start: i32, p2start: i32, die: Box<dyn Die>, win: i32) -> Self {
        Self {
            p1: Player {
                place: p1start,
                score: 0,
            },
            p2: Player {
                place: p2start,
                score: 0,
            },
            rolls: 0,
            die,
            win,
        }
    }

    fn turn1(&mut self) -> bool {
        let mut mv = 0;
        mv += self.die.roll();
        mv += self.die.roll();
        mv += self.die.roll();
        self.p1.place += mv;
        self.p1.place %= 10;
        if self.p1.place == 0 {
            self.p1.place = 10;
        }

        self.p1.score += self.p1.place;
        self.rolls += 3;

        self.p1.score >= self.win
    }

    fn turn2(&mut self) -> bool {
        let mut mv = 0;
        mv += self.die.roll();
        mv += self.die.roll();
        mv += self.die.roll();
        self.p2.place += mv;
        self.p2.place %= 10;
        if self.p2.place == 0 {
            self.p2.place = 10;
        }

        self.p2.score += self.p2.place;
        self.rolls += 3;

        self.p2.score >= self.win
    }
}

//part 2 infrastructure
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct GameDirac {
    p1score: i32,
    p1place: i32,
    p2score: i32,
    p2place: i32,
    win: i32,
}

impl GameDirac {
    fn new(p1start: i32, p2start: i32, win: i32) -> Self {
        Self {
            p1score: 0,
            p1place: p1start,
            p2score: 0,
            p2place: p2start,
            win,
        }
    }

    fn moven(&mut self, mv: i32, p1: bool) -> bool {
        if p1 {
            self.move1(mv)
        } else {
            self.move2(mv)
        }
    }

    fn move1(&mut self, mv: i32) -> bool {
        self.p1place += mv;
        self.p1place %= 10;
        if self.p1place == 0 {
            self.p1place = 10;
        }
        self.p1score += self.p1place;

        self.p1score >= self.win
    }

    fn move2(&mut self, mv: i32) -> bool {
        self.p2place += mv;
        self.p2place %= 10;
        if self.p2place == 0 {
            self.p2place = 10;
        }
        self.p2score += self.p2place;

        self.p2score >= self.win
    }
}

struct Game2 {
    games: HashMap<GameDirac, i64>,
    p1won: i64,
    p2won: i64,
}

impl Game2 {
    fn new(p1start: i32, p2start: i32, win: i32) -> Self {
        let mut games = HashMap::new();
        games.insert(GameDirac::new(p1start, p2start, win), 1);

        Self {
            games,
            p1won: 0,
            p2won: 0,
        }
    }

    fn step(&mut self, p1: bool) {
        let mut newgames = HashMap::new();

        for (game, count) in self.games.drain() {
            //1 game moves 3
            let mut gamemv = game.clone();
            if gamemv.moven(3, p1) {
                if p1 {
                    self.p1won += count * 1;
                } else {
                    self.p2won += count * 1;
                }
            } else {
                *newgames.entry(gamemv).or_insert(0) += count * 1;
            }

            //3 games move 4
            let mut gamemv = game.clone();
            if gamemv.moven(4, p1) {
                if p1 {
                    self.p1won += count * 3;
                } else {
                    self.p2won += count * 3;
                }
            } else {
                *newgames.entry(gamemv).or_insert(0) += count * 3;
            }

            //6 games move 5
            let mut gamemv = game.clone();
            if gamemv.moven(5, p1) {
                if p1 {
                    self.p1won += count * 6;
                } else {
                    self.p2won += count * 6;
                }
            } else {
                *newgames.entry(gamemv).or_insert(0) += count * 6;
            }

            //7 games move 6
            let mut gamemv = game.clone();
            if gamemv.moven(6, p1) {
                if p1 {
                    self.p1won += count * 7;
                } else {
                    self.p2won += count * 7;
                }
            } else {
                *newgames.entry(gamemv).or_insert(0) += count * 7;
            }

            //6 games move 7
            let mut gamemv = game.clone();
            if gamemv.moven(7, p1) {
                if p1 {
                    self.p1won += count * 6;
                } else {
                    self.p2won += count * 6;
                }
            } else {
                *newgames.entry(gamemv).or_insert(0) += count * 6;
            }

            //3 games move 8
            let mut gamemv = game.clone();
            if gamemv.moven(8, p1) {
                if p1 {
                    self.p1won += count * 3;
                } else {
                    self.p2won += count * 3;
                }
            } else {
                *newgames.entry(gamemv).or_insert(0) += count * 3;
            }

            //1 game moves 9
            let mut gamemv = game.clone();
            if gamemv.moven(9, p1) {
                if p1 {
                    self.p1won += count * 1;
                } else {
                    self.p2won += count * 1;
                }
            } else {
                *newgames.entry(gamemv).or_insert(0) += count * 1;
            }
        }

        self.games = newgames;
    }
}
