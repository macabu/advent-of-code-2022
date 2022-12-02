enum Point {
    One = 1,
    Two = 2,
    Three = 3,
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

enum Shape {
    Rock(Rock),
    Paper(Paper),
    Scissors(Scissors),
}

impl From<&str> for Shape {
    fn from(s: &str) -> Shape {
        match s {
            "A" | "X" => Shape::Rock(Rock {}),
            "B" | "Y" => Shape::Paper(Paper {}),
            "C" | "Z" => Shape::Scissors(Scissors {}),
            _ => unreachable!(),
        }
    }
}

enum EndRound {
    Win,
    Lose,
    Draw,
}

impl EndRound {
    fn into_shape(&self, against: &Shape) -> Shape {
        match (self, against) {
            (EndRound::Win, Shape::Rock(_)) => Shape::Paper(Paper {}),
            (EndRound::Win, Shape::Paper(_)) => Shape::Scissors(Scissors {}),
            (EndRound::Win, Shape::Scissors(_)) => Shape::Rock(Rock {}),
            (EndRound::Lose, Shape::Rock(_)) => Shape::Scissors(Scissors {}),
            (EndRound::Lose, Shape::Paper(_)) => Shape::Rock(Rock {}),
            (EndRound::Lose, Shape::Scissors(_)) => Shape::Paper(Paper {}),
            (EndRound::Draw, Shape::Rock(_)) => Shape::Rock(Rock {}),
            (EndRound::Draw, Shape::Paper(_)) => Shape::Paper(Paper {}),
            (EndRound::Draw, Shape::Scissors(_)) => Shape::Scissors(Scissors {}),
        }
    }
}

impl From<&str> for EndRound {
    fn from(s: &str) -> Self {
        match s {
            "X" => EndRound::Lose,
            "Y" => EndRound::Draw,
            "Z" => EndRound::Win,
            _ => unreachable!(),
        }
    }
}

trait Play {
    const POINTS: Point;

    fn outcome(&self, against: &Shape) -> Outcome;
    fn play(&self, against: &Shape) -> i32;
}

struct Rock;

impl Play for Rock {
    const POINTS: Point = Point::One;

    fn outcome(&self, against: &Shape) -> Outcome {
        match against {
            Shape::Rock(_) => Outcome::Draw,
            Shape::Paper(_) => Outcome::Lose,
            Shape::Scissors(_) => Outcome::Win,
        }
    }

    fn play(&self, against: &Shape) -> i32 {
        self.outcome(against) as i32 + Self::POINTS as i32
    }
}

struct Paper;

impl Play for Paper {
    const POINTS: Point = Point::Two;

    fn outcome(&self, against: &Shape) -> Outcome {
        match against {
            Shape::Rock(_) => Outcome::Win,
            Shape::Paper(_) => Outcome::Draw,
            Shape::Scissors(_) => Outcome::Lose,
        }
    }

    fn play(&self, against: &Shape) -> i32 {
        self.outcome(against) as i32 + Self::POINTS as i32
    }
}

struct Scissors;

impl Play for Scissors {
    const POINTS: Point = Point::Three;

    fn outcome(&self, against: &Shape) -> Outcome {
        match against {
            Shape::Rock(_) => Outcome::Lose,
            Shape::Paper(_) => Outcome::Win,
            Shape::Scissors(_) => Outcome::Draw,
        }
    }

    fn play(&self, against: &Shape) -> i32 {
        self.outcome(against) as i32 + Self::POINTS as i32
    }
}

fn sum_rounds(input: &str, shape_parser: fn(&str) -> Vec<Shape>) -> i32 {
    input
        .split("\n")
        .map(shape_parser)
        .map(|round| match &round[1] {
            Shape::Rock(r) => r.play(&round[0]),
            Shape::Paper(p) => p.play(&round[0]),
            Shape::Scissors(s) => s.play(&round[0]),
        })
        .sum::<i32>()
}

fn part_1(input: &str) -> i32 {
    sum_rounds(input, |line: &str| {
        line.split_whitespace()
            .map(|shape| Shape::from(shape))
            .collect::<Vec<_>>()
    })
}

fn part_2(input: &str) -> i32 {
    sum_rounds(input, |line| {
        let lines = line.split_whitespace().collect::<Vec<_>>();
        let against = Shape::from(lines[0]);
        let me = EndRound::from(lines[1]).into_shape(&against);
        vec![against, me]
    })
}

pub(crate) fn run() {
    let input = include_str!("../input/02.txt");

    println!("Day 2");
    println!("\tPart 1: {}", part_1(input));
    println!("\tPart 2: {}", part_2(input));
}
