use std::{fs::File, io::Read};

fn main() {
    let data = {
        let mut data = String::new();
        let mut file = File::open("./02/input.txt").unwrap();
        file.read_to_string(&mut data).unwrap();
        data
    };
    let outcome: i32 = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let line = l.split_once(' ').unwrap();
            let computer = RPS::new(line.0);
            let player = RPS::new(line.1);
            player.game_outcome(&computer).points() + player.points()
        })
        .sum();
    println!("Part One: {}", outcome);
    let outcome: i32 = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let line = l.split_once(' ').unwrap();
            let computer = RPS::new(line.0);
            let outcome = Outcome::new(line.1);
            outcome.points() + computer.for_outcome(outcome).points()
        })
        .sum();
    println!("Part Two: {}", outcome);
}

#[derive(Clone, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub fn new(char: &str) -> RPS {
        match char {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!("Invalid character"),
        }
    }

    pub fn points(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    pub fn game_outcome(&self, other: &RPS) -> Outcome {
        if self == other {
            return Outcome::Tie;
        }
        match self {
            RPS::Rock => match other {
                RPS::Paper => Outcome::Loss,
                RPS::Scissors => Outcome::Win,
                _ => panic!("Invalid character"),
            },
            RPS::Paper => match other {
                RPS::Rock => Outcome::Win,
                RPS::Scissors => Outcome::Loss,
                _ => panic!("Invalid character"),
            },
            RPS::Scissors => match other {
                RPS::Rock => Outcome::Loss,
                RPS::Paper => Outcome::Win,
                _ => panic!("Invalid character"),
            },
        }
    }

    pub fn for_outcome(&self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => match self {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
            Outcome::Loss => match self {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
            Outcome::Tie => self.clone(),
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Tie,
}

impl Outcome {
    pub fn new(char: &str) -> Outcome {
        match char {
            "X" => Outcome::Loss,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            _ => panic!("Invalid character"),
        }
    }

    pub fn points(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Tie => 3,
        }
    }
}
