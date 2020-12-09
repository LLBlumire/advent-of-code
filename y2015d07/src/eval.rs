use crate::*;
use std::{collections::HashMap,
          str::FromStr};

#[derive(Debug, Clone)]
enum Source {
    Gate(String),
    Number(i32),
}

#[derive(Debug, Clone)]
enum GateIn {
    Bind(Source),
    And(Source, Source),
    Or(Source, Source),
    RShift(Source, Source),
    LShift(Source, Source),
    Not(Source),
}

#[derive(Debug)]
pub struct ParsedInput {
    bindings: Vec<(String, GateIn)>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let number = |i| map(map_res(digit1, FromStr::from_str), Source::Number)(i);
    let gate = |i| map(alpha1, |a: &str| Source::Gate(a.to_string()))(i);
    let source = |i| alt((gate, number))(i);
    let in_bind = map(source, GateIn::Bind);
    let in_and = map(separated_pair(source, tag(" AND "), source), |(first, second)| GateIn::And(first, second));
    let in_or = map(separated_pair(source, tag(" OR "), source), |(first, second)| GateIn::Or(first, second));
    let in_lshift =
        map(separated_pair(source, tag(" LSHIFT "), source), |(first, second)| GateIn::LShift(first, second));
    let in_rshift =
        map(separated_pair(source, tag(" RSHIFT "), source), |(first, second)| GateIn::RShift(first, second));
    let in_not = map(tuple((tag("NOT "), source)), |(_, n)| GateIn::Not(n));
    let gate_in = alt((in_not, in_rshift, in_lshift, in_or, in_and, in_bind));
    let bind = tag(" -> ");
    let binding = map(separated_pair(gate_in, bind, alpha1), |(a, b)| (b.to_string(), a));
    let mut parsed = map(separated_list1(line_ending, binding), |bindings| ParsedInput { bindings });

    Ok(parsed(input)?)
}

pub struct WireSimulator {
    connections: HashMap<String, GateIn>,
    values: HashMap<String, i32>,
}

impl Source {
    fn get_from(&self, w: &mut WireSimulator) -> i32 {
        match self {
            Source::Gate(n) => w.get(n),
            Source::Number(n) => *n,
        }
    }
}

impl WireSimulator {
    fn new<I: IntoIterator<Item = (String, GateIn)>>(i: I) -> WireSimulator {
        WireSimulator { connections: i.into_iter().collect(), values: HashMap::new() }
    }

    fn get(&mut self, w: &str) -> i32 {
        if let Some(cache) = self.values.get(w) {
            return *cache;
        }
        let value = match self.connections.get(w).expect("Unknown string").clone() {
            GateIn::Bind(a) => a.get_from(self),
            GateIn::And(a, b) => a.get_from(self) & b.get_from(self),
            GateIn::Or(a, b) => a.get_from(self) | b.get_from(self),
            GateIn::RShift(a, b) => a.get_from(self) >> b.get_from(self),
            GateIn::LShift(a, b) => a.get_from(self) << b.get_from(self),
            GateIn::Not(a) => !a.get_from(self),
        };
        self.values.insert(w.to_string(), value);
        value
    }
}

pub type Task1 = i32;
pub type Task2 = i32;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let mut task1 = WireSimulator::new(input.bindings.clone());
    let task1_a = task1.get(&"a".to_string());
    let mut task2 = WireSimulator::new(input.bindings.into_iter().map(|(gate, input)| {
        if gate == "b" {
            (gate, GateIn::Bind(Source::Number(task1_a)))
        } else {
            (gate, input)
        }
    }));
    let task2_a = task2.get(&"a".to_string());
    Ok(Output { task1: task1_a, task2: task2_a })
}
