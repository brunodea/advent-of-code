use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Seat {
    Front,
    Back,
    Left,
    Right,
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            'F' => Seat::Front,
            'B' => Seat::Back,
            'L' => Seat::Left,
            'R' => Seat::Right,
            _ => unreachable!(),
        }
    }
}

impl Seat {
    fn step(&self, min: u32, max: u32) -> (u32, u32) {
        let half = (min as f32 + max as f32) / 2f32;
        match self {
            Seat::Front => (min, half.floor() as u32),
            Seat::Back => (half.ceil() as u32, max),
            Seat::Left => (min, half.floor() as u32),
            Seat::Right => (half.ceil() as u32, max),
        }
    }
}

#[derive(Debug, PartialEq)]
struct BoardingPass {
    row: u32,
    col: u32,
}

impl From<&str> for BoardingPass {
    fn from(s: &str) -> Self {
        assert_eq!(s.len(), 10);

        let steps = |min, max, pass: &str| -> u32 {
            let mut lhs = min;
            let mut rhs = max;
            for c in pass.chars() {
                let seat: Seat = c.into();
                let lr = seat.step(lhs, rhs);
                lhs = lr.0;
                rhs = lr.1;
            }
            assert_eq!(lhs, rhs);
            lhs
        };

        let row = steps(0, 127, &s[..7]);
        let col = steps(0, 7, &s[7..]);

        BoardingPass { row, col }
    }
}

impl BoardingPass {
    fn seat_id(&self) -> u32 {
        (self.row * 8) + self.col
    }
}

fn part1() {
    let stdin = io::stdin();
    let mut highest = 0u32;
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let boarding: BoardingPass = line.trim().into();
        let seat_id = boarding.seat_id();
        if seat_id > highest {
            highest = seat_id;
        }
    }

    println!("Highest seat ID: {}", highest);
}

fn part2() {
    let stdin = io::stdin();
    let mut boarding_passes = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let boarding: BoardingPass = line.trim().into();
        boarding_passes.push(boarding);
    }

    let mut seat_ids = boarding_passes.iter().map(|b| b.seat_id()).collect::<Vec<u32>>();
    seat_ids.sort();

    let mut index = 1usize;
    for seat_id in seat_ids[1..(seat_ids.len() - 2)].iter() {
        let has_prev = seat_ids[index - 1] == seat_id - 1;
        let has_next = seat_ids[index + 1] == seat_id + 1;
        if !has_prev {
            println!("No prev: {}", seat_id);
        }
        if !has_next {
            println!("No next: {}", seat_id);
        }
        index += 1;
    }
}

fn main() {
    // part1();
    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn steps_are_correct_front_back() {
        let boarding = "FBFBBFF".chars().map(|b| b.into()).collect::<Vec<Seat>>();
        assert_eq!(boarding.len(), 7);
        let mut boarding = boarding.iter();
        let step = boarding.next().unwrap().step(0, 127);
        assert_eq!(step, (0, 63));
        let step = boarding.next().unwrap().step(step.0, step.1);
        assert_eq!(step, (32, 63));
        let step = boarding.next().unwrap().step(step.0, step.1);
        assert_eq!(step, (32, 47));
        let step = boarding.next().unwrap().step(step.0, step.1);
        assert_eq!(step, (40, 47));
        let step = boarding.next().unwrap().step(step.0, step.1);
        assert_eq!(step, (44, 47));
        let step = boarding.next().unwrap().step(step.0, step.1);
        assert_eq!(step, (44, 45));
        let step = boarding.next().unwrap().step(step.0, step.1);
        assert_eq!(step, (44, 44));
    }

    #[test]
    fn steps_are_correct_left_right() {
        let boarding = "RLR".chars().map(|b| b.into()).collect::<Vec<Seat>>();
        assert_eq!(boarding.len(), 3);
        let mut boarding = boarding.iter();
        let step = boarding.next().unwrap().step(0, 7);
        assert_eq!(step, (4, 7));
        let step = boarding.next().unwrap().step(step.0, step.1);
        assert_eq!(step, (4, 5));
        let step = boarding.next().unwrap().step(step.0, step.1);
        assert_eq!(step, (5, 5));
    }

    #[test]
    fn converts_boarding_pass() {
        let boarding: BoardingPass = "BFFFBBFRRR".into();
        assert_eq!(boarding, BoardingPass { row: 70, col: 7});
        assert_eq!(boarding.seat_id(), 567);

        let boarding: BoardingPass = "FFFBBBFRRR".into();
        assert_eq!(boarding, BoardingPass { row: 14, col: 7});
        assert_eq!(boarding.seat_id(), 119);

        let boarding: BoardingPass = "BBFFBBFRLL".into();
        assert_eq!(boarding, BoardingPass { row: 102, col: 4});
        assert_eq!(boarding.seat_id(), 820);
    }
}
