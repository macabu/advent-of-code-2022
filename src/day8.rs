struct Coordinates(usize, usize);

struct Height(i32);

impl From<char> for Height {
    fn from(digit: char) -> Self {
        match str::parse::<i32>(&format!("{}", digit)) {
            Ok(height) if (0..=9).contains(&height) => Self(height),
            _ => unreachable!(),
        }
    }
}

struct Tree(Height, Coordinates);

impl Tree {
    const fn height(&self) -> i32 {
        self.0 .0
    }

    const fn x(&self) -> usize {
        self.1 .0
    }

    const fn y(&self) -> usize {
        self.1 .1
    }

    const fn in_corner(&self, map_size: usize) -> bool {
        match (self.x(), self.y()) {
            (0, _) => true,
            (_, 0) => true,
            (x, _) if x == map_size => true,
            (_, y) if y == map_size => true,
            _ => false,
        }
    }

    const fn is_adjacent_to(&self, another: &Self) -> bool {
        self.is_same_row_as(another) || self.is_same_column_as(another)
    }

    const fn is_same_row_as(&self, another: &Self) -> bool {
        self.x() != another.x() && self.y() == another.y()
    }

    const fn is_same_column_as(&self, another: &Self) -> bool {
        self.x() == another.x() && self.y() != another.y()
    }

    const fn is_taller_than(&self, another: &Self) -> bool {
        self.height() > another.height()
    }

    const fn is_shorter_than(&self, another: &Self) -> bool {
        self.height() < another.height()
    }

    fn grid_like_partition<'a>(
        &'a self,
        trees_map: &'a [Self],
    ) -> ((Vec<&Self>, Vec<&Self>), (Vec<&Self>, Vec<&Self>)) {
        let (same_row, same_col): (Vec<&Self>, Vec<&Self>) = trees_map
            .iter()
            .filter(|tree| self.is_adjacent_to(tree))
            .partition(|tree| self.is_same_row_as(tree));

        let (row_before, row_after): (Vec<&Self>, Vec<&Self>) =
            same_row.iter().partition(|tree| self.x() > tree.x());

        let (col_before, col_after): (Vec<&Self>, Vec<&Self>) =
            same_col.iter().partition(|tree| self.y() > tree.y());

        ((row_before, row_after), (col_before, col_after))
    }

    fn is_visible(&self, trees_map: &[Self]) -> bool {
        let (row, col) = self.grid_like_partition(trees_map);
        let (row_before, row_after) = row;
        let (col_before, col_after) = col;

        let row_before_visible = row_before.iter().all(|tree| tree.is_shorter_than(self));
        let row_after_visible = row_after.iter().all(|tree| tree.is_shorter_than(self));

        let col_before_visible = col_before.iter().all(|tree| tree.is_shorter_than(self));
        let col_after_visible = col_after.iter().all(|tree| tree.is_shorter_than(self));

        row_before_visible || row_after_visible || col_before_visible || col_after_visible
    }

    fn scenic_score(&self, trees_map: &[Self]) -> usize {
        let (row, col) = self.grid_like_partition(trees_map);
        let (row_before, row_after) = row;
        let (col_before, col_after) = col;

        let mut row_before_visible = row_before
            .iter()
            .rev()
            .take_while(|tree| self.is_taller_than(tree))
            .count();

        if row_before.len() != row_before_visible {
            row_before_visible += 1;
        }

        let mut row_after_visible = row_after
            .iter()
            .take_while(|tree| self.is_taller_than(tree))
            .count();

        if row_after.len() != row_after_visible {
            row_after_visible += 1;
        }

        let mut col_before_visible = col_before
            .iter()
            .rev()
            .take_while(|tree| self.is_taller_than(tree))
            .count();

        if col_before.len() != col_before_visible {
            col_before_visible += 1;
        }

        let mut col_after_visible = col_after
            .iter()
            .take_while(|tree| self.is_taller_than(tree))
            .count();

        if col_after.len() != col_after_visible {
            col_after_visible += 1;
        }

        row_before_visible * row_after_visible * col_before_visible * col_after_visible
    }
}

fn build_trees_map(input: &str) -> Vec<Tree> {
    let mut trees_map = Vec::<Tree>::new();

    input.lines().enumerate().for_each(|(i, line)| {
        line.char_indices().for_each(|(j, digit)| {
            let tree = Tree(Height::from(digit), Coordinates(i, j));
            trees_map.push(tree);
        });
    });

    trees_map
}

fn part_1(input: &str) -> usize {
    let map_size = input.lines().count();
    let trees_map = build_trees_map(input);

    let (in_corner, not_in_corner): (Vec<_>, Vec<_>) =
        trees_map.iter().partition(|tree| tree.in_corner(map_size));

    let total_visible_in_corners = in_corner.len();

    let total_visible_non_corners = not_in_corner
        .iter()
        .filter(|tree| tree.is_visible(&trees_map))
        .count();

    total_visible_non_corners + total_visible_in_corners
}

fn part_2(input: &str) -> usize {
    let map_size = input.lines().count();
    let trees_map = build_trees_map(input);

    trees_map
        .iter()
        .filter(|tree| !tree.in_corner(map_size))
        .map(|tree| tree.scenic_score(&trees_map))
        .max()
        .unwrap()
}

pub(crate) fn run() {
    let input = include_str!("../input/08.txt");

    println!("Day 8");
    println!("\tPart 1: {}", part_1(input));
    println!("\tPart 2: {}", part_2(input));
}
