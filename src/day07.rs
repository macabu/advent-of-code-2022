use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use petgraph::{graph::NodeIndex, visit::DfsPostOrder, Direction, Graph};

static SEP: &str = "~";
static DISK_SIZE_THRESHOLD: u32 = 100000;
static TOTAL_DISK_SIZE: u32 = 70000000;
static SPACE_NEEDED: u32 = 30000000;

#[derive(Debug)]
enum Directory {
    Back,
    Folder(String),
}

impl From<&str> for Directory {
    fn from(raw: &str) -> Self {
        match raw {
            ".." => Self::Back,
            dir => Self::Folder(dir.to_owned()),
        }
    }
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(Directory),
    List,
}

impl From<&str> for Command {
    fn from(raw_line: &str) -> Self {
        let mut raw_command = raw_line.split_whitespace().skip(1);

        match raw_command.next() {
            Some("ls") => Command::List,
            Some("cd") => match raw_command.next() {
                Some(dir) => Command::ChangeDirectory(Directory::from(dir)),
                None => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Output {
    Directory(Directory),
    File(String, u32),
}

impl From<&str> for Output {
    fn from(raw_line: &str) -> Self {
        let mut raw_output = raw_line.split_whitespace();

        match raw_output.next() {
            Some("dir") => match raw_output.next() {
                Some(dir) => Output::Directory(Directory::from(dir)),
                None => unreachable!(),
            },
            Some(size) if str::parse::<u32>(size).is_ok() => match str::parse::<u32>(size) {
                Ok(filesize) => match raw_output.next() {
                    Some(filename) => Output::File(filename.to_owned(), filesize),
                    None => unreachable!(),
                },
                Err(e) => panic!("{e}"),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum TerminalLine {
    Command(Command),
    Output(Vec<Output>),
}

#[derive(Debug, Default)]
struct Terminal(Vec<TerminalLine>);

impl Terminal {
    fn parse_raw_lines(input: &str) -> Self {
        let mut terminal = Self::default();

        input
            .lines()
            .group_by(|line| line.starts_with('$'))
            .into_iter()
            .for_each(|grouped_line| match grouped_line {
                (true, group) => {
                    group
                        .map(Command::from)
                        .for_each(|command| terminal.push(TerminalLine::Command(command)));
                }
                (false, group) => {
                    terminal.push(TerminalLine::Output(
                        group.map(Output::from).collect::<Vec<_>>(),
                    ));
                }
            });

        terminal
    }

    fn push(&mut self, item: TerminalLine) {
        self.0.push(item);
    }
}

type FileSystem = (Graph<String, u32>, Option<NodeIndex>);

impl From<Terminal> for FileSystem {
    fn from(terminal: Terminal) -> Self {
        let mut g = Graph::<String, u32>::new();
        let mut pwd: VecDeque<String> = VecDeque::new();
        let mut root_node_index: Option<_> = None;

        for line in &terminal.0 {
            match line {
                TerminalLine::Command(Command::ChangeDirectory(dir)) => match dir {
                    Directory::Back => {
                        if pwd.is_empty() {
                            continue;
                        }

                        pwd.pop_back();
                    }
                    Directory::Folder(dir) if dir == "/" => {
                        let node_index = g.add_node(dir.to_owned());

                        pwd.push_front(dir.to_owned());
                        root_node_index = Some(node_index);
                    }
                    Directory::Folder(dir) => {
                        let full_path = pwd.iter().join(SEP);
                        let new_full_path = full_path.clone() + SEP + dir;

                        if !g.node_weights().any(|node| *node == new_full_path) {
                            let to = g.add_node(new_full_path.to_owned());

                            if !pwd.is_empty() && g.node_weights().any(|node| *node == full_path) {
                                match g.node_indices().find(|node| g[*node] == *full_path) {
                                    Some(from) => {
                                        g.add_edge(from, to, 0);
                                    }
                                    None => unreachable!(),
                                }
                            }
                        }

                        pwd.push_back(dir.to_owned());
                    }
                },
                TerminalLine::Output(output) => {
                    for out in output {
                        if let Output::File(filename, filesize) = out {
                            let full_path = pwd.iter().join(SEP);
                            if let Some(from) = g.node_indices().find(|node| g[*node] == *full_path)
                            {
                                let new_full_path = full_path.clone() + SEP + filename;
                                if !g.node_weights().any(|node| *node == new_full_path) {
                                    let to = g.add_node(new_full_path.to_owned());
                                    g.add_edge(from, to, *filesize);
                                }
                            }
                        };
                    }
                }
                _ => (),
            }
        }

        (g, root_node_index)
    }
}

fn calculate_dir_size(input: &str) -> HashMap<String, u32> {
    let terminal = Terminal::parse_raw_lines(input);

    let (g, root_node_index) = FileSystem::from(terminal);
    // println!("{}", Dot::with_config(&g, &[]));

    let mut dir_sizes = HashMap::new();

    let mut dfs = DfsPostOrder::new(&g, root_node_index.unwrap());

    while let Some(node) = dfs.next(&g) {
        let mut edges = g.neighbors_directed(node, Direction::Incoming).detach();

        while let Some(edge) = edges.next_edge(&g) {
            if let Some(edge_weight) = g.edge_weight(edge) {
                let (f_idx, t_idx) = g.edge_endpoints(edge).unwrap();

                let from = g[f_idx].clone();
                let to = g[t_idx].clone();

                if edge_weight == &0 {
                    let sum_to = match dir_sizes.get(&to) {
                        Some(v) => *v,
                        None => {
                            panic!(
                                "No value found? from={} to={} weight={}",
                                from, to, edge_weight
                            );
                        }
                    };

                    match dir_sizes.get_mut(&from) {
                        Some(size_tally) => {
                            *size_tally += sum_to;
                        }
                        None => {
                            dir_sizes.insert(from, sum_to);
                        }
                    }

                    continue;
                }

                match dir_sizes.contains_key(&from) {
                    true => {
                        if let Some(size_tally) = dir_sizes.get_mut(&from) {
                            *size_tally += *edge_weight;
                        }
                    }
                    false => {
                        dir_sizes.insert(from, *edge_weight);
                    }
                }
            }
        }
    }

    dir_sizes
}

fn part_1(input: &str) -> u32 {
    calculate_dir_size(input)
        .iter()
        .filter(|dir| dir.1 <= &DISK_SIZE_THRESHOLD)
        .map(|dir| dir.1)
        .sum::<u32>()
}

fn part_2(input: &str) -> u32 {
    let folders = calculate_dir_size(input);

    let root_size = folders.get("/").unwrap();
    let unused_space = TOTAL_DISK_SIZE - root_size;
    let space_needed_for_update = SPACE_NEEDED - unused_space;

    folders
        .into_iter()
        .filter(|(_, size)| size >= &space_needed_for_update)
        .map(|(_, size)| size)
        .min()
        .unwrap()
}

pub(crate) fn run() {
    let input = include_str!("../input/07.txt");

    println!("Day 07");
    println!("\tPart 1: {}", part_1(input));
    println!("\tPart 2: {}", part_2(input));
}
