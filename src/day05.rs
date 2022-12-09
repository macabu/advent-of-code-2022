use std::collections::VecDeque;

#[derive(Debug)]
struct Pop {
    from: usize,
    count: u8,
}

#[derive(Debug)]
struct Push {
    from: usize,
    count: u8,
}

#[derive(Debug)]
enum Operation {
    Push(Push),
    Pop(Pop),
}

enum CraneKind {
    Single,
    Multiple,
}

impl Operation {
    fn from_str(input: &str, kind: &CraneKind) -> VecDeque<Self> {
        let mut raw_instruction = input.split(' ');

        let _move = raw_instruction.next().unwrap();

        let move_count = raw_instruction
            .next()
            .map(|n| n.parse::<u8>().unwrap())
            .unwrap();

        let _from = raw_instruction.next().unwrap();

        let from_crate = raw_instruction
            .next()
            .map(|n| n.parse::<usize>().unwrap())
            .unwrap();

        let _to = raw_instruction.next().unwrap();

        let to_crate = raw_instruction
            .next()
            .map(|n| n.parse::<usize>().unwrap())
            .unwrap();

        let mut operations = VecDeque::new();

        match kind {
            CraneKind::Single => {
                for _ in 0..move_count {
                    operations.push_back(Operation::Pop(Pop {
                        from: from_crate - 1,
                        count: 1,
                    }));
                }

                for _ in 0..move_count {
                    operations.push_back(Operation::Push(Push {
                        from: to_crate - 1,
                        count: 1,
                    }));
                }
            }
            CraneKind::Multiple => {
                operations.push_back(Operation::Pop(Pop {
                    from: from_crate - 1,
                    count: move_count,
                }));

                operations.push_back(Operation::Push(Push {
                    from: to_crate - 1,
                    count: move_count,
                }));
            }
        }

        operations
    }
}

#[derive(Debug)]
struct Stack(VecDeque<u8>);

impl Stack {
    fn new() -> Self {
        Self(VecDeque::new())
    }

    fn push_front(&mut self, item: u8) {
        self.0.push_front(item)
    }

    fn push_back(&mut self, item: u8) {
        self.0.push_back(item)
    }

    fn pop_front(&mut self) -> Option<u8> {
        self.0.pop_front()
    }
}

struct Stacks(Vec<Stack>);

impl Stacks {
    fn build_from_lines(parsed_lines: &VecDeque<VecDeque<Option<u8>>>) -> Self {
        let mut stacks = std::iter::repeat_with(Stack::new)
            .take(parsed_lines.len() + 1)
            .collect::<Vec<_>>();

        for line in parsed_lines {
            for (idx, item) in line.iter().enumerate() {
                if let Some(item) = item {
                    stacks[idx].push_back(*item);
                }
            }
        }

        Self(stacks)
    }

    fn run_operations(&mut self, operations: &VecDeque<Operation>) {
        let mut stack_pointer: VecDeque<u8> = VecDeque::new();

        for operation in operations {
            match operation {
                Operation::Push(Push {
                    from: idx,
                    count: n,
                }) => {
                    for _ in 0..*n {
                        if let Some(sp) = stack_pointer.pop_back() {
                            self.0[*idx].push_front(sp);
                        }
                    }
                }
                Operation::Pop(Pop {
                    from: idx,
                    count: 1,
                }) => {
                    if let Some(sp) = self.0[*idx].pop_front() {
                        stack_pointer.push_front(sp);
                    }
                }
                Operation::Pop(Pop {
                    from: idx,
                    count: n,
                }) => {
                    let mut temp = vec![];

                    for _ in 0..*n {
                        if let Some(sp) = self.0[*idx].pop_front() {
                            temp.push(sp);
                        }
                    }

                    temp.reverse();
                    temp.iter().for_each(|item| stack_pointer.push_front(*item));
                }
            }
        }
    }

    fn top_of_stacks(&self) -> String {
        let mut top = String::new();

        for stack in &self.0 {
            let item = stack.0[0];
            top.push(char::from_u32(item as u32).unwrap());
        }

        top
    }
}

fn parse_lines(stack: &[&str]) -> VecDeque<VecDeque<Option<u8>>> {
    let mut parsed_lines = stack
        .iter()
        .map(|line| {
            line.as_bytes()
                .chunks(4)
                .map(|chunk| {
                    let mut new_chunk = chunk;

                    if chunk.len() == 4 {
                        new_chunk = &new_chunk[..3];
                    }

                    match &new_chunk[1] {
                        n if n.is_ascii_alphabetic() => Some(*n),
                        _ => None,
                    }
                })
                .collect::<VecDeque<_>>()
        })
        .collect::<VecDeque<_>>();

    parsed_lines.pop_back();

    parsed_lines
}

fn expand_top_of_stacks(input: &str, crane_kind: CraneKind) -> String {
    let raw_stack_lines = input
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let parsed_lines = parse_lines(&raw_stack_lines);
    let mut stacks = Stacks::build_from_lines(&parsed_lines);

    let operations = input
        .lines()
        .skip(raw_stack_lines.len() + 1)
        .flat_map(|line| Operation::from_str(line, &crane_kind))
        .collect::<VecDeque<_>>();

    stacks.run_operations(&operations);

    stacks.top_of_stacks()
}

fn part_1(input: &str) -> String {
    expand_top_of_stacks(input, CraneKind::Single)
}

fn part_2(input: &str) -> String {
    expand_top_of_stacks(input, CraneKind::Multiple)
}

pub(crate) fn run() {
    let input = include_str!("../input/05.txt");

    println!("Day 05");
    println!("\tPart 1: {}", part_1(input));
    println!("\tPart 2: {}", part_2(input));
}
