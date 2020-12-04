#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Open,
    Tree,
    BuiltOpen,
    BuiltTree,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        if c == '#' {
            Cell::Tree
        } else {
            Cell::Open
        }
    }
}

impl Cell {
    pub fn to_char(&self) -> char {
        match *self {
            Cell::Open => '.',
            Cell::Tree => '#',
            Cell::BuiltOpen => 'O',
            Cell::BuiltTree => 'X',
        }
    }
}

struct Row(Vec<Cell>);

impl From<&str> for Row {
    fn from(line: &str) -> Self {
        let mut r = Vec::with_capacity(line.len());
        for c in line.chars() {
            r.push(c.into());
        }

        Row(r)
    }
}

impl Row {
    pub fn cell(&self, pos: usize) -> Cell {
        self.0[pos % self.0.len()]
    }

    pub fn build(&mut self, pos: usize) {
        let index = pos % self.0.len();
        self.0[index] = match self.0[index] {
            Cell::Open => Cell::BuiltOpen,
            Cell::Tree => Cell::BuiltTree,
            _ => unreachable!(),
        }
    }

    pub fn print(&self) {
        for cell in self.0.iter() {
            print!("{}", cell.to_char());
        }
        println!();
    }
}

struct Forest {
    rows: Vec<Row>,
}

impl From<&str> for Forest {
    fn from(raw: &str) -> Self {
        let rows = raw
            .split('\n')
            .map(|line| line.into())
            .collect::<Vec<Row>>();
        Forest { rows }
    }
}

impl Forest {
    pub fn print(&self) {
        for row in self.rows.iter() {
            row.print();
        }
    }

    pub fn part1(&self) -> u32 {
        let num_rows = self.rows.len();
        let mut row = 1usize;
        let mut col = 3usize;
        let mut num_trees = 0;
        while row < num_rows {
            let cell = self.rows[row].cell(col);
            if cell == Cell::Tree {
                num_trees += 1;
            }
            row += 1;
            col += 3;
        }

        num_trees
    }

    fn part2_aux(&mut self, right: usize, down: usize) -> u32 {
        let num_rows = self.rows.len();
        let mut row = down;
        let mut col = right;
        let mut num_trees = 0;
        while row < num_rows {
            let cell = self.rows[row].cell(col);
            if cell == Cell::Tree {
                num_trees += 1;
            }
            // TODO(brunor): remove line below
            // self.rows[row].build(col);
            row += down;
            col += right;
        }

        num_trees
    }

    fn part2(&mut self) -> usize {
        let inputs = [
            (1usize, 1usize),
            (3usize, 1usize),
            (5usize, 1usize),
            (7usize, 1usize),
            (1usize, 2usize),
        ];
        let mut res = 1usize;
        for (right, down) in inputs.iter() {
            let v = self.part2_aux(*right, *down) as usize;
            println!("right: {} down: {} = {}", right, down, v);
            res *= v;
        }
        // self.print();
        res
    }
}

fn main() {
    let tile = ".#.#....##.......#..........#..
...#...........##...#..#.......
#.####......##.#...#......#.#..
##.....#.#.#..#.#............#.
##.....#....#.........#...##...
###..#.....#....#..............
..........#..#.#..#.#....#.....
##.....#....#.#...#.##.........
#...#......#....##....#..#.#...
.##.##...#....##..#.#.....#...#
.....#.#..........##.#........#
.##..................#..#..##.#
#.#..........##....#.####......
.#......#.#......#.........#...
#....#..##.##..##........#.#...
##..#.##..#...#..####.#..#.....
###....#.###.##...........##..#
.....#.##.....##.#..#####....##
....#.###....#..##....##...#...
..###.#...##.....#.##..#..#.#..
#...#..#..#.........#..#.......
##..#.#.....#.#.#.......#...#.#
...#...##.#........#...#.......
..#..#.#..#...#...#...........#
........#.....#......#...##....
#........##.##.#.#...#...#.....
####.......#.##.###.#....#.....
...#...........#...#......#...#
##...#...#............#.......#
....#...........##.......#.....
###......#.....#....#...#.#...#
.....##..........#.......#.#...
##.##.##...#......#....#.......
##..#.#..#......#...#..#.......
....#....##.##............####.
..#.###..#.##.###..#.##.......#
#.##..#.#.....#..#.....##......
..##..#.....##.#.##........#...
.#..#.#......#..#............#.
.....#..#.#...#....#.##.#......
.#...##.#..#.#...##...##..##...
###............#.#..#..#...#...
..#..##.####.#.....#.....##.###
#....#.##..##....#..#...#.##.#.
.....#.##.........##...##......
.........####.#....#.#......#.#
.........#.#..#...#.#..#.#....#
.#.....#..##.##..##....#.......
..........##......#.##.###....#
.##...###..##.#...#........##..
..............#.#....#.#.###.##
..##.##.......#.#......##...#..
.#.....#..##..#.###...#..#.##.#
#.....#.#..#...#........#...#..
.#......#....#.#.....###...#..#
..##.#....#..##......#.....#...
..#.#.##..#.....#.####..###....
.........#......#..#...........
..#........#.##.#.....##.##..#.
.......#.........#....#...#.#..
.##.....#.#....#.#.......#.....
..........#.##........##...##..
###..###.#.#..#..#####.##.#.##.
..##..##.#.#...#..#.#.#......#.
#..#..#..#..##..#.....#......#.
..#....#.##..#......##.........
..#.##......#...##.#......#....
.......#..#.##.#.....#.........
.......#.#.#.###...##......#...
.....#.#..........#..#...#.....
....##..........#..........##..
..#......#.....#.##.#..#...#.#.
....#.....#..#...#..#.#.##..###
.####....#........#...#........
...##.#.##.#..#...##...#.##....
....#...#...#.#.#.#...#..#.....
.....#...#.#.....#.#........##.
..#.#.......###.#.....##.......
......#.........##....#....#..#
.............##.....##.........
.........##...##.......#.....#.
##.........#..........#.###..##
...#.....#......#....#..##.....
##..#...#...##.#.....#.#......#
..#...##.#.......#.#......#.##.
......#.......#.#...........#..
..........#.....##............#
#........#...#..#.......###.##.
.##...........#.#........#.#.#.
...#..##...#.#....#####.#......
.....##...###...#..#.##...####.
...#....#.....#..#.......#.....
#....#....#...#..#..#.######..#
#.###...........#......#...#..#
.#.#.#.#..#....#....#...##.#...
.#..#.........#.#....###...#...
......#..##.##..........#....##
.....#......##....##.....#...#.
.#...#.#.#....##....#..#....#.#
..................#..###.#..##.
..#.........#......#....#..###.
#.#.....#..#..#....###..###....
..##..##.#..##........##...##..
##..#........##..###..#.....#.#
..#..###..#......#....#...#...#
#..#.#..............##.#..#.#..
.....####....#...####.....#.#..
.....#....##.#......###........
##.##...#.#.#.#.......#....##..
.#......#...#.#....#..##.#.##.#
#.#.##.#.#......#..##........##
...##.....#.....#...#..###...#.
........###.....#.....#...##..#
.....#.##.##......#.#....#...#.
.#....##.......#..#.####.......
.#..#....#..........#......#.#.
.#.##.##.....###.#.#...........
.........#......#..##..........
....#...##.#.#.#..#.#.........#
..#.....#.##...#..#..#.###....#
...#.##......#.....##....#.....
###............#.#....#...#....
.......#.....#..#.#.#....#..#.#
...#......#.#..##..#....#...#.#
............##........##..##...
..#..#.##..#......###..#.......
........#.........#............
..#...#.#########.#...##..###..
#....#......#.......#.#.....#..
#.#..#....###.###....#...#.#...
#...###.#.#.......#.##......#..
.................#...#.#.#.....
##....#...#........#....#.#..#.
......#.....#...#..........#.#.
##..........#...#..........#.##
..#.#.##.#....#.#......#...##..
.....#.......#..#.....#........
#.##.#..##..#.......##.........
....#......#..#..#.#...#.......
...#....#................###...
.##.....#.#....#.#..........##.
...#..#....#.##.##......#......
..#.#....#.......#.#..##.......
....#.....#..........##.#.#####
#.....................##..#..#.
.###..#.##.......##.#...#..#...
...###.......#..#...#......#..#
#..#...#.#..#.#..#..#.##.......
#...##.......#..#..#.##..###...
......#....#.#.#........#.##..#
..##..#....#....#..#.#..#......
..##.#...#.#######..#...#.....#
..#....#..#.........#..##......
...#....#.#......#..#..#.#.....
#..#....#........#.#..##....###
#....#..##......##.##.....#.###
...#.#..........#..#.#.#.#.##..
......##..#.#..#.#....#....#...
##....#....#..#..#.##......#...
....#.#..##.#.#...###....##.#..
...#.......##..#.......#...#...
......##.......#..##.....#...#.
...#.#...#...........#...#.....
.#....#...#......##.##..###..#.
.#..........#...#...#...##.##..
.....###..#.....#..##....#.####
..#.###..#..##..##.....#.#.....
.............#.###...##.#.....#
....###.......###.#.....#..#.#.
........##.#.........#.....###.
.....###.#..#.....#...#..#.....
.#....#..##.#..#.#....#.......#
........#......#.#..#.#..#...##
...#.##.##......#..............
.#.....##.#.....#..#......##...
#..#..#.....#.....#.....###....
.##...........#..#.##.....#....
..#.#......#.#...#.##.#..#...##
...#..........#.....#..........
#.#.#.#.#...#....#...#.....##..
#......##...#...#..........#.#.
....##........#.#..............
#..#.#.#..#........##......#.##
........####...##.#.....#......
....#........#.#..#..##..#.#...
.#.....#..###...#..#.....#..#..
#......###.#..#....#..#.#......
....#.....##.##..#...#.#..##.#.
..##..#...#.#......#....#...#.#
#..##...##..#...###...#..#.....
.......#.....#...........##....
#..##....#........#....##..#.#.
.#........#..##...###.#..#.....
.#.#....#..##...#...##.#..###..
#.........#.......#.....#.#....
#..#.....#.#.###.#..#......#...
....#..#.#....#..##..###....###
###.##.#.#..#...........#.#.#..
..##.#.......#......#..##....#.
.....#.#.#.......##.......#...#
...........#.##....##.##....#.#
...#.......#..#.##..#......#..#
#.#.#...#......##.#...........#
##........#...........###.#..#.
..........#.#.#....#.#..##.#.#.
...#.#.#....#..........#..#....
#.#....###.#.#..#.......###...#
.#....#......#.#.#..#..#.......
......##.............#....#.#.#
.#..........#.........#.##.....
##....#....##....#..#.......#..
#.##.##.#..#..#.....#..#.##.#..
.#..#.......##..#.....##.##....
.......#..........#.#.##..#.##.
....#.....#.#...##....##.......
.......#.........#...##....##.#
#.....#......#..........#...#..
...#.#.......#.#..#....###..#..
.....#.#.#.........#...........
.#..###.#.#........#.#.........
.........#..#......##...##....#
...###..#.....##.....#.###....#
.##...#...#........###.#..#....
.##........#..#.###.######.##.#
##.#...#.#....#..##.#....##....
.......##.....##.#..###.#......
..##...##........#.......#....#
#..##...#.####...###......#...#
.##.....#.##.#.#.....###.#..##.
..###....#.#.###.#....#........
....#..###..#...#....#..#..#.#.
#.#.##....##...##.......#......
.........#...#....#..#.........
.............#...#..##.#.......
...#.##.......#...#.#..##.##...
.####.#.##..#.#......#.##...#.#
.#..#.#.....#.................#
..#.##..###....#...#......####.
..##..##...........#....#...#..
....#...#...#...#.......#....#.
#.#...###...#...#.#...#....##.#
......#...#.#.......#.....#...#
....##...#.#.#....#....#.#....#
.....#.....#...##..#...#....##.
#.....#....#......##.##....#...
...#.#....#...#....#.#....##..#
...#.#..#...##....###..#.......
...##......###...###.#...#..#..
##.......#.......###.......#..#
..##.##..###.#............#...#
#.....##..#..##....##..#.......
......#.#...#......#.....#.....
#...........#....#..##.##.#....
.......#..#......#...#....#...#
.#...##...........#......#...#.
#........#....##...###.#....#..
.....#.......##.........#.##...
.#.###..#....#..##.#..#.#..#...
#.......#.##.#.#....#.#..#....#
###.....#.#.......#..#......#.#
#..#.#.......#.#..##..##.#.#...
#..#.#.#.###........#.....#...#
#.#.#..#..##.....#...........#.
..#.#..#.....#...#...#...##....
...#.##......#...##.#...#.#.#.#
#..#.#.#.#.......####..........
..#......#.#......##.###.....##
..#...##..#.........##....#.##.
##.##.##.#.#.....#..........##.
.#.....###.#..#....#..#.###...#
#...##.......###....#.#..#.....
..#....##.........##.........##
......#....#.##.......#........
..#.#.#..#...#...#...##.#...#..
......#..##.#.#.#...##...#.#.##
#..#...##.#.....#...#.##.......
..#..#.........##.#...#.##...##
##.##.#....#.......#.##..#.....
.....##...##.##...##.........##
#......#...#.......#...#...#...
...##...........#...#..#.......
.#.##.#..#........#....#.......
#.#...#..#......##...#.#.##....
##........####..#.#...#.#.##.##
#..#.#.##......##.#.#..#.......
.....#.........#..#.####....#..
......##..#....#...#.#....#....
#...##........#.........#.....#
.#.#...#.#.#..#............##.#
.#..#....#....#.....#...#.....#
..###...#..#.....#.##.###...#.#
.#.###..#..#...#.#...#.#......#
#...#####......###........##...
.....#.....#..#.#....#..##.....
....##...#.#.##.#####...#....#.
.#.#.........##.#.......#..##..
.#...#.#...#...#....#.#...##.#.
.##...#..#.#..#......#.#.#..##.
..#.....#..#.....##.....#......
..#........#..##...#.......###.
.#....#.......#....#....#..#...
....#......#.#.#.........#.....
..##...#.#.#...#.#........#....
.#.....####...##.#..#...##.....
...#.....#...#...#....#....#...
.........#..#.#.....#..#.#..#..
.........##...........#.......#
......#..#.....##...#.##.#.....
.#......##........##...#.#.##..
.....#.#..##...........#..#..#.
...#.......#...#.#..#.##..#.##.
...#.......#.....#.#...#.##.#..
#.....#.............##.#..####.
.#...#......#...##.#....#.#....
.##..##.##....#.#.....#.......#
...#...#....#....##.#..#....##.
..............##....#.......#.#
.#.#.#...##..#..#...###.#..#...
.#.#...#.#..#.#..#...######..#.
........#......#.#..#.#....#...
..###.....###.#.##....#...##...
.##.#.....#.......##.......#...
..#..##...#..........#.#....#.#";
    let mut trees: Forest = tile.into();
    println!("Part 1: {}", trees.part1());
    println!("Part 2: {}", trees.part2());
}