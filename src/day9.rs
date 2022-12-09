use std::{borrow::Borrow, collections::HashSet, rc::Rc};

#[derive(Debug)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
    Diagonally(Rc<Motion>, Rc<Motion>),
}

impl From<&str> for Motion {
    fn from(raw_motion: &str) -> Self {
        match raw_motion {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Coordinates(i32, i32);

impl Coordinates {
    const fn x(&self) -> i32 {
        self.0
    }

    const fn y(&self) -> i32 {
        self.1
    }

    const fn is_tailing(&self, another: &Coordinates) -> bool {
        let horizontal_distance = self.x().abs_diff(another.x());
        let vertical_distance = self.y().abs_diff(another.y());

        match (horizontal_distance, vertical_distance) {
            (0, 0) => true, // overlapping
            (0, 1) => true,
            (1, 0) => true,
            (1, 1) => true,
            _ => false,
        }
    }

    fn define_motion(&self, another: &Coordinates) -> Motion {
        let horizontal_distance = self.x() - another.x();
        let vertical_distance = self.y() - another.y();

        match (horizontal_distance, vertical_distance) {
            // ..H.T..
            (2, 0) => Motion::Left,

            // ..T.H..
            (-2, 0) => Motion::Right,

            // ...T...
            // .......
            // ...H...
            (0, 2) => Motion::Down,

            // ...H...
            // .......
            // ...T...
            (0, -2) => Motion::Up,

            // ...T..
            // ......
            // ..H...
            (n, 2) if n > 0 => Motion::Diagonally(Rc::new(Motion::Left), Rc::new(Motion::Down)),

            // ..T...
            // H.....
            // ......
            (2, n) if n > 0 => Motion::Diagonally(Rc::new(Motion::Left), Rc::new(Motion::Down)),

            // ......
            // H.....
            // ..T...
            (2, n) if n < 0 => Motion::Diagonally(Rc::new(Motion::Left), Rc::new(Motion::Up)),

            // H.....
            // ......
            // .T....
            (n, -2) if n > 0 => Motion::Diagonally(Rc::new(Motion::Left), Rc::new(Motion::Up)),

            // T.....
            // ......
            // .H....
            (n, 2) if n < 0 => Motion::Diagonally(Rc::new(Motion::Right), Rc::new(Motion::Down)),

            // ......
            // T.....
            // ..H...
            (-2, n) if n > 0 => Motion::Diagonally(Rc::new(Motion::Right), Rc::new(Motion::Down)),

            // ..H...
            // ......
            // .T....
            (n, -2) if n < 0 => Motion::Diagonally(Rc::new(Motion::Right), Rc::new(Motion::Up)),

            // ..H...
            // T.....
            // ......
            (-2, n) if n < 0 => Motion::Diagonally(Rc::new(Motion::Right), Rc::new(Motion::Up)),

            _ => unreachable!(),
        }
    }

    fn move_up(&mut self) {
        self.1 += 1
    }

    fn move_down(&mut self) {
        self.1 -= 1
    }

    fn move_left(&mut self) {
        self.0 -= 1
    }

    fn move_right(&mut self) {
        self.0 += 1
    }

    fn move_to(&mut self, motion: &Motion) {
        match motion {
            Motion::Up => self.move_up(),
            Motion::Down => self.move_down(),
            Motion::Left => self.move_left(),
            Motion::Right => self.move_right(),
            Motion::Diagonally(horizontal, vertical) => {
                let horizontal = horizontal.borrow();
                self.move_to(horizontal);

                let vertical = vertical.borrow();
                self.move_to(vertical);
            }
        };
    }
}

#[derive(Debug)]
struct Knot(Coordinates);

impl Knot {
    const fn pos(&self) -> &Coordinates {
        &self.0
    }

    fn set_pos(&mut self) -> &mut Coordinates {
        &mut self.0
    }
}

fn calculate_steps_recorded(input: &str, knot_len: i32) -> i32 {
    let motions = input
        .lines()
        .flat_map(|motion| {
            let mut m = motion.split_whitespace();
            let motion_type = m.next().unwrap();
            let motion_count = str::parse::<i32>(m.next().unwrap()).unwrap();

            (0..motion_count).map(move |_| Motion::from(motion_type))
        })
        .collect::<Vec<_>>();

    let mut head = Knot(Coordinates(0, 0));
    let mut knots = (1..=knot_len)
        .map(|_| Knot(Coordinates(0, 0)))
        .collect::<Vec<_>>();
    let mut steps_recorded = HashSet::new();

    for motion in &motions {
        head.set_pos().move_to(motion);

        let mut another_pos = head.pos().clone();

        for (i, knot) in knots.iter_mut().enumerate() {
            if knot.pos().is_tailing(&another_pos) {
                break;
            }

            let knot_motion = knot.pos().define_motion(&another_pos);

            knot.set_pos().move_to(&knot_motion);

            another_pos = knot.pos().clone();

            if i as i32 == knot_len - 1 {
                steps_recorded.insert(knot.pos().clone());
            }
        }
    }

    steps_recorded.len() as i32
}

fn part_1(input: &str) -> i32 {
    calculate_steps_recorded(input, 1)
}

fn part_2(input: &str) -> i32 {
    calculate_steps_recorded(input, 9) + 1
}

pub(crate) fn run() {
    let input = include_str!("../input/09.txt");

    println!("Day 9");
    println!("\tPart 1: {}", part_1(input));
    println!("\tPart 2: {}", part_2(input));
}
