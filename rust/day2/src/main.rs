use std::{error, str::FromStr};

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = include_str!("./input.txt");

    let part1 = input
        .lines()
        .map(|line| {
            let line = line.split(" ").collect::<Vec<&str>>();
            let opponent = line[0].parse::<Movement>().expect("Invalid movement");
            let me = line[1].parse::<Movement>().expect("Invalid movement");
            me.points() + me.duel(&opponent)
        })
        .sum::<usize>();

    let part2 = input
        .lines()
        .map(|line| {
            let line = line.split(" ").collect::<Vec<&str>>();
            let opponent = line[0].parse::<Movement>().expect("Invalid movement");
            let me = Movement::from_desired_outcome(&opponent, line[1]);
            me.points() + me.duel(&opponent)
        })
        .sum::<usize>();

    println!("# Day 2: Rock Paper Scissors");
    println!("Part 1 = {part1}");
    println!("Part 2 = {part2}");
    Ok(())
}

#[derive(Clone, PartialEq)]
enum Movement {
    Paper,
    Rock,
    Scissors,
}

impl FromStr for Movement {
    type Err = Box<dyn error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => panic!("Invalid move value"),
        }
    }
}

impl Movement {
    fn from_desired_outcome(opponent: &Self, outcome: &str) -> Self {
        return match (opponent, outcome) {
            (_, "X") => opponent.wins_against(),
            (_, "Y") => opponent.clone(),
            (_, "Z") => opponent.loses_against(),
            _ => panic!("Invalid movement or outcome value"),
        };
    }

    fn wins_against(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn loses_against(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn points(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn duel(&self, other: &Self) -> usize {
        match (self, other) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => 6,
            _ if other == self => 3,
            _ => 0,
        }
    }
}
