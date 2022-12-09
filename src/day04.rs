use itertools::Itertools;

struct SectionRange(u8, u8);

impl SectionRange {
    fn from_raw(range: &str) -> Self {
        let section = range.split("-").collect::<Vec<_>>();
        let begin = str::parse::<u8>(section[0]).expect("begin section parse");
        let end = str::parse::<u8>(section[1]).expect("end section parse");

        Self(begin, end)
    }
}

struct Section(SectionRange, SectionRange);

impl Section {
    fn from_raw(section: &str) -> Self {
        let ranges = section.split(",").collect::<Vec<_>>();
        let left = SectionRange::from_raw(ranges[0]);
        let right = SectionRange::from_raw(ranges[1]);

        Self(left, right)
    }

    fn fully_overlaps(&self) -> bool {
        let SectionRange(left_begin, left_end) = self.0;
        let SectionRange(right_begin, right_end) = self.1;

        match (left_begin, right_begin) {
            _ if left_begin == right_begin => true,
            _ if left_begin > right_begin => right_end >= left_end,
            _ if left_begin < right_begin => right_end <= left_end,
            _ => unreachable!(),
        }
    }

    fn any_overlaps(&self) -> bool {
        let SectionRange(left_begin, left_end) = self.0;
        let SectionRange(right_begin, right_end) = self.1;

        ![
            (left_begin..=left_end).collect::<Vec<_>>(),
            (right_begin..=right_end).collect::<Vec<_>>(),
        ]
        .concat()
        .iter()
        .all_unique()
    }
}

fn count_overlaps(input: &str, overlapper: fn(&Section) -> bool) -> i32 {
    input
        .lines()
        .map(Section::from_raw)
        .filter(overlapper)
        .count() as i32
}

fn part_1(input: &str) -> i32 {
    count_overlaps(input, |section| section.fully_overlaps())
}

fn part_2(input: &str) -> i32 {
    count_overlaps(input, |section| section.any_overlaps())
}

pub(crate) fn run() {
    let input = include_str!("../input/04.txt");

    println!("Day 04");
    println!("\tPart 1: {}", part_1(input));
    println!("\tPart 2: {}", part_2(input));
}
