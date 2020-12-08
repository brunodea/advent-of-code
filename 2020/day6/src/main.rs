use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Answer(char);

impl From<char> for Answer {
    fn from(c: char) -> Self {
        Answer(c)
    }
}

#[derive(Debug)]
struct Person {
    answers: HashSet<Answer>,
}

impl From<&str> for Person {
    fn from(s: &str) -> Self {
        let answers = s.chars().map(|c| c.into()).collect::<HashSet<Answer>>();
        Person { answers: answers }
    }
}

#[derive(Debug)]
struct Group {
    people: Vec<Person>,
}

impl Group {
    fn new() -> Self {
        Group { people: Vec::new() }
    }

    fn unique_answers_anyone(&self) -> HashSet<Answer> {
        self.people
            .iter()
            .map(|p| p.answers.iter().cloned().collect())
            .fold(HashSet::new(), |lhs, rhs| {
                lhs.union(&rhs).cloned().collect()
            })
    }

    fn unique_answers_everyone(&self) -> HashSet<Answer> {
        self.people
            .iter()
            .map(|p| Some(p.answers.iter().cloned().collect()))
            .fold(None, |lhs: Option<HashSet<Answer>>, rhs: Option<HashSet<Answer>>| {
                if let Some(lhs) = lhs {
                    let r = Some(lhs.intersection(&rhs.unwrap()).cloned().collect());
                    r
                } else {
                    rhs
                }
            })
            .unwrap_or(HashSet::new())
    }
}

fn part1() {
    let stdin = io::stdin();
    let mut groups = vec![Group::new()];
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        if line.len() == 0 {
            groups.push(Group::new());
        } else {
            let len = groups.len();
            groups[len - 1].people.push(line[..].into());
        }
    }

    let counts = groups.iter().map(|g| g.unique_answers_anyone().len()).fold(0, |l, r| l + r);

    println!("Part1 answer: {}", counts);
}

fn part2() {
    let stdin = io::stdin();
    let mut groups = vec![Group::new()];
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        if line.len() == 0 {
            groups.push(Group::new());
        } else {
            let len = groups.len();
            groups[len - 1].people.push(line[..].into());
        }
    }

    let counts = groups.iter().map(|g| g.unique_answers_everyone().len()).fold(0, |l, r| l + r);

    println!("Part2 answer: {}", counts);
}

fn main() {
    //part1();
    part2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_person() {
        let p: Person = "abc".into();
        assert_eq!(p.answers.len(), 3);
        assert!(p.answers.contains(&Answer('a')));
        assert!(p.answers.contains(&Answer('b')));
        assert!(p.answers.contains(&Answer('c')));
    }

    #[test]
    fn correct_unique_answers_everyone() {
        let people: [Person; 3] = ["abc".into(), "abe".into(), "abde".into()];
        let group = Group { people: Vec::from(people) };
        let unique = group.unique_answers_everyone();
        assert_eq!(unique.len(), 2);
        assert!(unique.contains(&Answer('a')));
        assert!(unique.contains(&Answer('b')));
    }
}
