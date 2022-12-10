use std::{collections::HashMap, fmt::Display};

trait CyclesTaken<const CYCLES_TAKEN: i32> {
    fn cycles_taken(&self) -> i32 {
        CYCLES_TAKEN
    }
}

#[derive(Debug)]
struct Noop;

impl CyclesTaken<1> for Noop {}

#[derive(Debug)]
struct Addx(i32);

impl CyclesTaken<2> for Addx {}

#[derive(Debug)]
enum Instruction {
    Noop(Noop),
    Addx(Addx),
}

impl From<&str> for Instruction {
    fn from(raw_instruction: &str) -> Self {
        let mut instruction = raw_instruction.split_whitespace();

        match instruction.next() {
            Some("noop") => Self::Noop(Noop),
            Some("addx") => {
                let value = instruction
                    .next()
                    .map(|i| str::parse::<i32>(i).expect("parsing instruction to i32"))
                    .expect("fetching instruction value");

                Self::Addx(Addx(value))
            }
            _ => unreachable!(),
        }
    }
}

const MILESTONES: &[i32; 6] = &[20, 60, 100, 140, 180, 220];

#[derive(Debug)]
struct Cpu {
    register_value: i32,
    cycle_counter: i32,
    cycle_register: HashMap<i32, i32>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            register_value: 1,
            cycle_counter: 1,
            cycle_register: Default::default(),
        }
    }
}

impl Cpu {
    fn update_cycle(&mut self, cycles: i32) {
        if cycles != 0 {
            self.cycle_register
                .insert(self.cycle_counter, self.register_value);

            self.cycle_counter += 1;

            self.update_cycle(cycles - 1);
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop(noop) => {
                self.update_cycle(noop.cycles_taken());
            }
            Instruction::Addx(addx) => {
                self.update_cycle(addx.cycles_taken());
                self.register_value += addx.0;
            }
        }
    }

    fn signal_strength(&self, milestone: &i32) -> i32 {
        match self.cycle_register.get(milestone) {
            Some(register_value) => milestone * register_value,
            None => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Pixel {
    Light,
    Dark,
}

impl From<Pixel> for char {
    fn from(p: Pixel) -> Self {
        match p {
            Pixel::Light => '#',
            Pixel::Dark => '.',
        }
    }
}

impl Pixel {
    fn for_sprite(cycle_count: i32, register_value: i32) -> Self {
        let sprite_positions = if register_value - 1 < 0 {
            [0, 0, 0]
        } else {
            [register_value - 1, register_value, register_value + 1]
        };

        match sprite_positions.contains(&(cycle_count - 1)) {
            true => Pixel::Light,
            false => Pixel::Dark,
        }
    }
}

struct CrtLine(Vec<char>);

impl Display for CrtLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .fold(Ok(()), |acc, pixel| acc.and_then(|_| write!(f, "{pixel}")))
    }
}

impl CrtLine {
    fn draw_pixel(&mut self, pixel: Pixel) {
        self.0.push(char::from(pixel));
    }
}

struct Crt(Vec<CrtLine>);

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .fold(Ok(()), |acc, line| acc.and_then(|_| writeln!(f, "{line}")))
    }
}

impl Crt {
    fn draw_pixel(&mut self, cycle_count: i32, register_value: i32) {
        let (crt_line_index, crt_line_offset) = match cycle_count {
            1..=40 => (0, 0),
            41..=80 => (1, 40),
            81..=120 => (2, 80),
            121..=160 => (3, 120),
            161..=200 => (4, 160),
            201..=240 => (5, 200),
            _ => unreachable!(),
        };

        let pixel = Pixel::for_sprite(cycle_count, crt_line_offset + register_value);

        self.0[crt_line_index].draw_pixel(pixel);
    }

    fn draw_all(&mut self, cycle_register: &HashMap<i32, i32>) {
        for cycle_count in 1..=cycle_register.len() {
            let register_value = match cycle_register.get(&(cycle_count as i32)) {
                Some(n) => n,
                None => unreachable!(),
            };

            self.draw_pixel(cycle_count as i32, *register_value);
        }
    }
}

fn run_cpu_instructions(input: &str) -> Cpu {
    let instructions = input.lines().map(Instruction::from).collect::<Vec<_>>();

    let mut cpu = Cpu::default();

    instructions
        .iter()
        .for_each(|instruction| cpu.run_instruction(instruction));

    cpu
}

fn part_1(input: &str) -> i32 {
    let cpu = run_cpu_instructions(input);

    MILESTONES
        .iter()
        .map(|milestone| cpu.signal_strength(milestone))
        .sum::<i32>()
}

fn part_2(input: &str) -> Crt {
    let cpu = run_cpu_instructions(input);

    let mut crt = Crt((0..6)
        .map(|_| CrtLine(Vec::with_capacity(40)))
        .collect::<Vec<_>>());

    crt.draw_all(&cpu.cycle_register);

    crt
}

pub(crate) fn run() {
    let input = include_str!("../input/10.txt");

    println!("Day 10");
    println!("\tPart 1: {}", part_1(input));
    println!("\tPart 2:\n{}", part_2(input));
}
