use crate::*;
use std::{collections::BTreeMap,
          str::FromStr};

#[derive(Debug)]
pub struct ParsedInput {
    list: Vec<usize>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let number = |i| map_res(digit1, FromStr::from_str)(i);
    let mut parsed = map(separated_list1(char(','), number), |list| ParsedInput { list });
    Ok(parsed(input)?)
}

#[derive(Debug)]
struct ElfGameSim {
    full_list: Vec<usize>,
    last_position: BTreeMap<usize, usize>,
}
impl ElfGameSim {
    fn tail(&self) -> Option<usize> {
        self.full_list.last().copied()
    }
    fn find_in_self(&self, element: usize) -> Option<usize> {
        self.last_position.get(&element).copied()
    }
    fn insert(&mut self, element: usize) {
        if let Some(tail) = self.tail() {
            self.last_position.insert(tail, self.full_list.len());
        }
        self.full_list.push(element);
    }
    fn populate_next(&mut self) {
        if let Some(last) = self.tail() {
            self.insert(self.find_in_self(last).map(|n| self.full_list.len() - n).unwrap_or_default());
        }
    }
    fn simulate_to(init: &[usize], limit: usize) -> ElfGameSim {
        let mut sim = ElfGameSim { full_list: Vec::new(), last_position: BTreeMap::new() };
        for &item in init {
            sim.insert(item);
        }
        while sim.full_list.len() < limit {
            sim.populate_next();
        }
        sim
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: ElfGameSim::simulate_to(&input.list, 2020).tail().unwrap(),
        task2: ElfGameSim::simulate_to(&input.list, 30000000).tail().unwrap(),
    })
}
