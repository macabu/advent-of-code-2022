use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Rhs {
    Scalar(u128),
    Itself,
}

impl From<&str> for Rhs {
    fn from(raw_rhs: &str) -> Self {
        match str::parse::<u128>(raw_rhs) {
            Ok(scalar) => Self::Scalar(scalar),
            Err(_) => Self::Itself,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Rhs),
    Sub(Rhs),
    Mul(Rhs),
}

impl From<&str> for Operation {
    fn from(raw_operation: &str) -> Self {
        let mut op_lines = raw_operation.split_whitespace().skip(4);

        let op = op_lines.next();

        let rhs = match op_lines.next() {
            Some(rhs) => Rhs::from(rhs),
            None => unreachable!(),
        };

        match op {
            Some("+") => Self::Add(rhs),
            Some("-") => Self::Sub(rhs),
            Some("*") => Self::Mul(rhs),
            n => unreachable!("{:?}", n),
        }
    }
}

impl Operation {
    fn eval(&self, lhs: &WorryLevel) -> WorryLevel {
        let WorryLevel(old) = lhs;

        let new = match self {
            Operation::Add(rhs) => match rhs {
                Rhs::Scalar(scalar) => old + scalar,
                _ => unreachable!(),
            },
            Operation::Sub(rhs) => match rhs {
                Rhs::Scalar(scalar) => old - scalar,
                _ => unreachable!(),
            },
            Operation::Mul(rhs) => match rhs {
                Rhs::Scalar(scalar) => old * scalar,
                Rhs::Itself => old * old,
            },
        };

        WorryLevel(new)
    }
}

#[derive(Debug)]
enum TestBranch {
    True(MonkeyID),
    False(MonkeyID),
}

impl From<&str> for TestBranch {
    fn from(raw_test_branch: &str) -> Self {
        let mut test_line = raw_test_branch.split_whitespace().skip(1);

        let cond = test_line.next();

        let dest = match test_line.nth(3) {
            Some(n) => match str::parse::<u32>(n) {
                Ok(monkey) => monkey,
                Err(_) => unreachable!(),
            },
            _ => unreachable!(),
        };

        match cond {
            Some("true:") => Self::True(MonkeyID(dest)),
            Some("false:") => Self::False(MonkeyID(dest)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct TestCondDivisible(u128);

impl From<&str> for TestCondDivisible {
    fn from(raw_test_cond: &str) -> Self {
        match raw_test_cond.split_whitespace().last() {
            Some(number) => match str::parse::<u128>(number) {
                Ok(n) => Self(n),
                Err(_) => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl TestCondDivisible {
    fn divisible(&self, lhs: WorryLevel) -> bool {
        let WorryLevel(wl) = lhs;
        let TestCondDivisible(divisor) = self;

        (wl as f64 % *divisor as f64) == 0.
    }
}

#[derive(Debug)]
struct Test {
    cond: TestCondDivisible,
    branch_true: TestBranch,
    branch_false: TestBranch,
}

impl From<&str> for Test {
    fn from(raw_test: &str) -> Self {
        let mut test_lines = raw_test.lines();

        let cond = TestCondDivisible::from(test_lines.next().unwrap());
        let branch_true = TestBranch::from(test_lines.next().unwrap());
        let branch_false = TestBranch::from(test_lines.next().unwrap());

        assert!(test_lines.next().is_none());

        Self {
            cond,
            branch_true,
            branch_false,
        }
    }
}

impl Test {
    fn eval(&self, worry_level: WorryLevel) -> MonkeyID {
        if let true = self.cond.divisible(worry_level) {
            if let TestBranch::True(monkey_id) = &self.branch_true {
                return *monkey_id;
            }
        } else {
            if let TestBranch::False(monkey_id) = &self.branch_false {
                return *monkey_id;
            }
        }

        unreachable!()
    }
}

#[derive(Debug, Clone, Copy)]
struct WorryLevel(u128);

impl From<&str> for WorryLevel {
    fn from(raw_worry_level: &str) -> Self {
        match str::parse::<u128>(raw_worry_level) {
            Ok(v) => {
                assert!(v > 0);

                WorryLevel(v)
            }
            Err(_) => unreachable!(),
        }
    }
}

impl WorryLevel {
    const BOREDNESS_FACTOR: f64 = 3.;

    // Thanks internet, I would not have figured this out otherwise.
    const MULTIPLY_ALL_DIV: u128 = 11 * 2 * 5 * 7 * 17 * 19 * 3 * 13;

    fn manage(&mut self, management: &WorryLevelManagement) {
        match management {
            WorryLevelManagement::Bored => {
                self.0 = (self.0 as f64 / Self::BOREDNESS_FACTOR).floor() as u128;
            }
            WorryLevelManagement::FigureItOut => {
                self.0 %= Self::MULTIPLY_ALL_DIV;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct MonkeyID(u32);

impl From<&str> for MonkeyID {
    fn from(raw_monkey_id: &str) -> Self {
        match raw_monkey_id.split_whitespace().nth(1) {
            Some(id) => match str::parse::<u32>(id.trim_end_matches(':')) {
                Ok(id) => MonkeyID(id),
                Err(e) => unreachable!("found {e}, {id}"),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Items(VecDeque<WorryLevel>);

impl From<&str> for Items {
    fn from(raw_starting_items: &str) -> Self {
        let raw_items = raw_starting_items.split_whitespace().skip(2);

        let worry_levels = raw_items
            .map(|raw_item| raw_item.trim_end_matches(','))
            .map(WorryLevel::from)
            .collect::<VecDeque<_>>();

        Self(worry_levels)
    }
}

#[derive(Debug)]
struct Monkey {
    id: MonkeyID,
    items: Items,
    operation: Operation,
    test: Test,
}

impl From<&str> for Monkey {
    fn from(raw_monkey: &str) -> Self {
        let mut raw_monkey_lines = raw_monkey.lines();

        let id = MonkeyID::from(raw_monkey_lines.next().unwrap());
        let items = Items::from(raw_monkey_lines.next().unwrap());
        let operation = Operation::from(raw_monkey_lines.next().unwrap());
        let test = Test::from(raw_monkey_lines.collect::<Vec<_>>().join("\n").as_ref());

        Self {
            id,
            items,
            operation,
            test,
        }
    }
}

enum WorryLevelManagement {
    Bored,
    FigureItOut,
}

struct Round {
    item_queue: HashMap<MonkeyID, Items>,
    inspections: HashMap<MonkeyID, u128>,
}

impl Round {
    fn run(&mut self, _round_num: u32, monkeys: &mut [Monkey], management: &WorryLevelManagement) {
        // println!("Round {:?}", _round_num);

        for monkey in monkeys.iter_mut() {
            // println!("\t{:?}:", &monkey.id);

            if let Some(items) = self.item_queue.get_mut(&monkey.id) {
                while !items.0.is_empty() {
                    monkey.items.0.push_back(items.0.pop_front().unwrap());
                }
            }

            for item in &monkey.items.0 {
                if let Some(inspection) = self.inspections.get_mut(&monkey.id) {
                    *inspection += 1;
                }
                // println!("\t\tMonkey inspects an item with a {:?}", &item);

                let mut new = monkey.operation.eval(item);
                // println!("\t\t\tWorry level is {:?} to {:?}.",&monkey.operation, &new);

                new.manage(management);
                // println!("\t\t\After managing {:?}.", &new);

                let target_monkey_id = monkey.test.eval(new);
                // println!("\t\t\tItem with {:?} is thrown to monkey {:?}", &new, &target_monkey_id);

                let target_queue = self.item_queue.get_mut(&target_monkey_id).unwrap();

                target_queue.0.push_back(new);
            }

            monkey.items.0.clear()
        }
    }
}

fn monkey_business(input: &str, rounds: u32, managed_level: WorryLevelManagement) -> u128 {
    let mut monkeys = input.split("\n\n").map(Monkey::from).collect::<Vec<_>>();

    let item_queue = (0..monkeys.len()).map(|monkey_id| {
        (
            MonkeyID(monkey_id.try_into().unwrap()),
            Items(VecDeque::new()),
        )
    });

    let inspections =
        (0..monkeys.len()).map(|monkey_id| (MonkeyID(monkey_id.try_into().unwrap()), 0));

    let mut round = Round {
        item_queue: HashMap::from_iter(item_queue),
        inspections: HashMap::from_iter(inspections),
    };

    for round_num in 1..=rounds {
        round.run(round_num, &mut monkeys, &managed_level);
    }

    let mut top_inspections = round.inspections.iter().collect::<Vec<_>>();
    top_inspections.sort_by(|(_, a), (_, b)| b.cmp(a));

    top_inspections
        .iter()
        .take(2)
        .map(|(_, count)| count)
        .fold(1, |acc, &count| acc * count)
}

fn part_1(input: &str) -> u128 {
    monkey_business(input, 20, WorryLevelManagement::Bored)
}

fn part_2(input: &str) -> u128 {
    monkey_business(input, 10_000, WorryLevelManagement::FigureItOut)
}

pub(crate) fn run() {
    let input = include_str!("../input/11.txt");

    println!("Day 11");
    println!("\tPart 1: {}", part_1(input));
    println!("\tPart 2: {}", part_2(input));
}
