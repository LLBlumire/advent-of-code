use std::collections::HashMap;

use aoc::*;

#[derive(Debug)]
struct ParsedInput<'a> {
    bindings: Vec<(&'a str, GateIn<'a>)>,
}

#[derive(Copy, Clone, Debug)]
enum Source<'a> {
    Wire(&'a str),
    Number(u16),
}

#[derive(Copy, Clone, Debug)]
enum GateIn<'a> {
    Bind(Source<'a>),
    And(Source<'a>, Source<'a>),
    Or(Source<'a>, Source<'a>),
    RShift(Source<'a>, Source<'a>),
    LShift(Source<'a>, Source<'a>),
    Not(Source<'a>),
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, line_ending, u16},
        combinator::map,
        multi::separated_list1,
        sequence::{separated_pair, tuple},
    };
    let number = |i| map(u16, Source::Number)(i);
    let gate = |i| map(alpha1, Source::Wire)(i);
    let source = |i| alt((gate, number))(i);
    let in_bind = map(source, GateIn::Bind);
    let in_and = map(
        separated_pair(source, tag(" AND "), source),
        |(first, second)| GateIn::And(first, second),
    );
    let in_or = map(
        separated_pair(source, tag(" OR "), source),
        |(first, second)| GateIn::Or(first, second),
    );
    let in_lshift = map(
        separated_pair(source, tag(" LSHIFT "), source),
        |(first, second)| GateIn::LShift(first, second),
    );
    let in_rshift = map(
        separated_pair(source, tag(" RSHIFT "), source),
        |(first, second)| GateIn::RShift(first, second),
    );
    let in_not = map(tuple((tag("NOT "), source)), |(_, n)| GateIn::Not(n));
    let gate_in = alt((in_not, in_rshift, in_lshift, in_or, in_and, in_bind));
    let bind = tag(" -> ");
    let binding = map(separated_pair(gate_in, bind, alpha1), |(a, b)| (b, a));
    let bindings = separated_list1(line_ending, binding);
    let mut parse = map(bindings, |bindings| ParsedInput { bindings });
    parse(input)
}

#[derive(Default)]
struct WireSystem<'a> {
    connections: HashMap<&'a str, GateIn<'a>>,
    resolved_values: HashMap<&'a str, u16>,
}
impl<'a> FromIterator<(&'a str, GateIn<'a>)> for WireSystem<'a> {
    fn from_iter<T: IntoIterator<Item = (&'a str, GateIn<'a>)>>(iter: T) -> Self {
        WireSystem {
            connections: iter.into_iter().collect(),
            resolved_values: HashMap::new(),
        }
    }
}
impl<'a> WireSystem<'a> {
    fn resolve_source(&mut self, source: Source<'a>) -> Result<u16> {
        Ok(match source {
            Source::Wire(wire) => self.resolve_wire(wire)?,
            Source::Number(number) => number,
        })
    }
    fn resolve_wire(&mut self, wire: &'a str) -> Result<u16> {
        if let Some(&cache) = self.resolved_values.get(wire) {
            return Ok(cache);
        }
        let value = match *self.connections.get(wire).ok_or("Unknown wire")? {
            GateIn::Bind(a) => self.resolve_source(a)?,
            GateIn::And(a, b) => self.resolve_source(a)? & self.resolve_source(b)?,
            GateIn::Or(a, b) => self.resolve_source(a)? | self.resolve_source(b)?,
            GateIn::RShift(a, b) => self.resolve_source(a)? >> self.resolve_source(b)?,
            GateIn::LShift(a, b) => self.resolve_source(a)? << self.resolve_source(b)?,
            GateIn::Not(a) => !self.resolve_source(a)?,
        };
        self.resolved_values.insert(wire, value);
        Ok(value)
    }
}

fn task1(input: &ParsedInput) -> Result<u16> {
    WireSystem::from_iter(input.bindings.iter().copied()).resolve_wire("a")
}

fn task2(input: &ParsedInput, task1: u16) -> Result<u16> {
    WireSystem::from_iter(input.bindings.iter().map(|&(gate, input)| {
        if gate == "b" {
            (gate, GateIn::Bind(Source::Number(task1)))
        } else {
            (gate, input)
        }
    }))
    .resolve_wire("a")
}

#[test]
fn test() {
    assert_task!(parse, task1, "123 -> x\n456 -> y\nx AND y -> a", 72)
}

aoc_main!(parse, task1 -> task2);
