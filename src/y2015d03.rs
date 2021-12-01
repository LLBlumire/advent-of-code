use aoc::*;
use std::collections::BTreeSet;

struct ParsedInput {
    directions: Vec<(i64, i64)>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        branch::alt,
        character::complete::char,
        combinator::{map, value},
        multi::many1,
    };
    let north = value((0, -1), char('^'));
    let east = value((1, 0), char('>'));
    let south = value((0, 1), char('v'));
    let west = value((-1, 0), char('<'));
    let direciton = alt((north, east, south, west));
    let directions = many1(direciton);
    let mut parse = map(directions, |directions| ParsedInput { directions });
    parse(input)
}

#[derive(Default)]
pub struct SantaSimulator {
    position: (i64, i64),
    presents: BTreeSet<(i64, i64)>,
}
impl SantaSimulator {
    fn do_move(&mut self, &(dx, dy): &(i64, i64)) -> &mut Self {
        self.position.0 += dx;
        self.position.1 += dy;
        self
    }
    fn do_gift(&mut self) -> &mut Self {
        self.presents.insert(self.position);
        self
    }
    fn simulate<'a>(&mut self, directions: impl IntoIterator<Item = &'a (i64, i64)>) -> &mut Self {
        self.do_gift();
        for direction in directions {
            self.do_move(direction).do_gift();
        }
        self
    }
    fn reset_position(&mut self) -> &mut Self {
        self.position = Default::default();
        self
    }
    fn visited_houses(&self) -> usize {
        self.presents.len()
    }
}

fn task1(input: &ParsedInput) -> Result<usize> {
    Ok(SantaSimulator::default()
        .simulate(input.directions.iter())
        .visited_houses())
}

fn task2(input: &ParsedInput) -> Result<usize> {
    Ok(SantaSimulator::default()
        .simulate(input.directions.iter().step_by(2))
        .reset_position()
        .simulate(input.directions.iter().skip(1).step_by(2))
        .visited_houses())
}

#[test]
fn test() {
    assert_task!(parse, task1, ">", 2);
    assert_task!(parse, task1, "^>v<", 4);
    assert_task!(parse, task1, "^v^v^v^v^v", 2);
    assert_task!(parse, task2, "^v", 3);
    assert_task!(parse, task2, "^>v<", 3);
    assert_task!(parse, task2, "^v^v^v^v^v", 11);
}

aoc_main!(parse, task1, task2);
