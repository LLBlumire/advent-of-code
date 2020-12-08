use crate::*;
use std::collections::{BTreeMap,
                       BTreeSet};

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct ParsedInput {
    directions: Vec<Direction>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let north = map(char('^'), |_| Direction::North);
    let east = map(char('>'), |_| Direction::East);
    let south = map(char('v'), |_| Direction::South);
    let west = map(char('<'), |_| Direction::West);
    let direction = alt((north, east, south, west));
    let mut parsed = map(many1(direction), |directions| ParsedInput { directions });
    Ok(parsed(input)?)
}

#[derive(Debug, Default)]
pub struct SantaSimulator {
    position: (i64, i64),
    presents: BTreeMap<(i64, i64), i64>,
}
impl SantaSimulator {
    fn do_move(&mut self, direction: &Direction) {
        self.position = match (direction, self.position) {
            (Direction::North, (x, y)) => (x, y - 1),
            (Direction::East, (x, y)) => (x + 1, y),
            (Direction::South, (x, y)) => (x, y + 1),
            (Direction::West, (x, y)) => (x - 1, y),
        }
    }
    fn do_gift(&mut self) {
        *self.presents.entry(self.position).or_insert(0) += 1;
    }
    fn simulate<'a, I: Iterator<Item = &'a Direction>>(directions: I) -> SantaSimulator {
        let mut sim = SantaSimulator::default();
        sim.do_gift();
        for direction in directions {
            sim.do_move(direction);
            sim.do_gift();
        }
        sim
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: SantaSimulator::simulate(input.directions.iter()).presents.iter().count(),
        task2: {
            let santa = SantaSimulator::simulate(input.directions.iter().step_by(2));
            let robot = SantaSimulator::simulate(input.directions.iter().skip(1).step_by(2));
            let santa_visits: BTreeSet<&(i64, i64)> = santa.presents.keys().collect();
            let robot_visits: BTreeSet<&(i64, i64)> = robot.presents.keys().collect();
            santa_visits.union(&robot_visits).count()
        },
    })
}
