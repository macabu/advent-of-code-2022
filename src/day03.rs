use itertools::iproduct;

#[derive(Clone, PartialEq, Eq)]
struct Item(u8);

impl Item {
    fn new(c: char) -> Self {
        Self(c as u8)
    }

    fn priority(&self) -> i32 {
        match self.0 {
            65..=90 => 27 + (self.0 - 65) as i32,
            97..=122 => 1 + (self.0 - 97) as i32,
            _ => unreachable!(),
        }
    }
}

struct Rucksack(Vec<Item>);

impl Rucksack {
    fn new(items: &[Item]) -> Self {
        Rucksack(items.to_vec())
    }

    fn items(&self) -> &Vec<Item> {
        &self.0
    }

    fn equal_item(&self) -> Option<&Item> {
        let (left, right) = self.items().split_at(self.items().len() / 2);

        iproduct!(left, right)
            .find(|(l, r)| l == r)
            .map(|(l, _r)| l)
    }
}

fn make_rucksacks(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|rucksack| {
            Rucksack::new(
                &rucksack
                    .chars()
                    .map(|item| Item::new(item))
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn part_1(input: &str) -> i32 {
    make_rucksacks(input)
        .iter()
        .map(|rucksack| rucksack.equal_item().map(|item| item.priority()).unwrap())
        .sum()
}

fn part_2(input: &str) -> i32 {
    make_rucksacks(input)
        .chunks_exact(3)
        .map(|group| {
            let (f, s, t) = (&group[0], &group[1], &group[2]);

            iproduct!(f.items(), s.items(), t.items())
                .find(|(f, s, t)| f == s && s == t)
                .map(|(f, _, _)| f.priority())
                .unwrap()
        })
        .sum()
}

pub(crate) fn run() {
    let input = include_str!("../input/03.txt");

    println!("Day 03");
    println!("\tPart 1: {}", part_1(input));
    println!("\tPart 2: {}", part_2(input));
}
