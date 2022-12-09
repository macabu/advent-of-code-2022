fn part_1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|backpack| {
            backpack
                .split("\n")
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .fold(0, |sum, calorie| sum + calorie)
        })
        .max()
        .unwrap()
}

fn part_2(input: &str) -> i32 {
    let mut total_calories = input
        .split("\n\n")
        .map(|backpack| {
            backpack
                .split("\n")
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .fold(0, |sum, calorie| sum + calorie)
        })
        .collect::<Vec<_>>();

    total_calories.sort();

    total_calories
        .get(total_calories.len() - 3..total_calories.len())
        .unwrap()
        .into_iter()
        .fold(0, |sum, calorie| sum + calorie)
}

pub(crate) fn run() {
    let input = include_str!("../input/01.txt");

    println!("Day 01");
    println!("\tPart 1: {}", part_1(input));
    println!("\tPart 2: {}", part_2(input));
}
