use itertools::Itertools;

enum Marker {
    StartOfPacket = 4,
    StartOfMessage = 14,
}

fn find_first_marker_pos(input: &str, marker: Marker) -> i32 {
    input
        .char_indices()
        .collect::<Vec<_>>()
        .as_slice()
        .windows(marker as usize)
        .find(|window| window.iter().map(|(_, c)| c).all_unique())
        .map(|chars| chars.last().map(|(idx, _)| *idx as i32 + 1).unwrap_or(-1))
        .unwrap_or(-1)
}

fn part_1(input: &str) -> i32 {
    find_first_marker_pos(input, Marker::StartOfPacket)
}

fn part_2(input: &str) -> i32 {
    find_first_marker_pos(input, Marker::StartOfMessage)
}

pub(crate) fn run() {
    let input = include_str!("../input/06.txt");

    println!("Day 06");
    println!("\tPart 1: {}", part_1(input));
    println!("\tPart 2: {}", part_2(input));
}
