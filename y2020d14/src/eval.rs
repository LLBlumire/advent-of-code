use crate::*;
use std::{collections::BTreeMap,
          str::FromStr};

#[derive(Debug)]
enum Command {
    MaskSet(String),
    MemSet(u64, u64),
}

#[derive(Debug)]
pub struct ParsedInput {
    commands: Vec<Command>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let number = |i| map_res(digit1, FromStr::from_str)(i);
    let mask_set = map(tuple((tag("mask = "), alphanumeric1)), |(_, n): (_, &str)| Command::MaskSet(n.to_string()));
    let mem_set =
        map(tuple((delimited(tag("mem["), number, tag("] = ")), number)), |(loc, set)| Command::MemSet(loc, set));
    let command = alt((mask_set, mem_set));
    let mut parsed = map(separated_list1(line_ending, command), |commands| ParsedInput { commands });
    Ok(parsed(input)?)
}

fn mask_from_number(input: u64) -> Vec<bool> {
    (0..36u32).map(|n| (input >> n) & 1 == 1).collect()
}
fn number_from_mask(input: &[bool]) -> u64 {
    input.iter().enumerate().map(|(i, b)| if *b { 2u64.pow(i as u32) } else { 0 }).sum::<u64>()
        & 0x0000_000f_ffff_ffff_u64
}
fn mask_ones(number: &[bool], set_mask: &[bool]) -> Vec<bool> {
    number.iter().zip(set_mask.iter()).map(|(&bit, &mask_bit)| mask_bit || bit).collect()
}
fn mask_zeros(number: &[bool], set_mask: &[bool]) -> Vec<bool> {
    number.iter().zip(set_mask.iter()).map(|(&bit, &mask_bit)| !mask_bit && bit).collect()
}

#[derive(Debug)]
pub struct Sim {
    memory: BTreeMap<u64, u64>,
    mask_zero: Vec<bool>,
    mask_one: Vec<bool>,
}
impl Sim {
    fn blank() -> Sim {
        Sim { memory: BTreeMap::new(), mask_zero: vec![false; 36], mask_one: vec![false; 36] }
    }
    fn set_masks_from_str(&mut self, mask: &str) {
        self.mask_one = vec![false; 36];
        self.mask_zero = vec![false; 36];
        for (i, c) in mask.bytes().rev().enumerate() {
            match c {
                b'X' => {}
                b'1' => self.mask_one[i] = true,
                b'0' => self.mask_zero[i] = true,
                _ => panic!(),
            }
        }
    }
    fn set_memory(&mut self, address: u64, value: u64) {
        let value = mask_from_number(value);
        let value = mask_zeros(&value, &self.mask_zero);
        let value = mask_ones(&value, &self.mask_one);
        let value = number_from_mask(&value);
        self.memory.insert(address, value);
    }
    fn simulate(commands: &[Command]) -> Sim {
        let mut sim = Sim::blank();
        for command in commands {
            match command {
                Command::MaskSet(mask) => sim.set_masks_from_str(&mask),
                Command::MemSet(address, value) => sim.set_memory(*address, *value),
            }
        }
        sim
    }
}

fn gen_mask_float(seed: u64, set_mask: &[bool]) -> Vec<(bool, bool)> {
    let seed = mask_from_number(seed);
    let mut seed = seed.iter();
    set_mask
        .iter()
        .map(|mask_bit| match mask_bit {
            true => match seed.next().unwrap() {
                true => (true, false),
                false => (false, true),
            },
            false => (false, false),
        })
        .collect()
}

fn mask_floats(number: &[bool], set_mask: &[bool]) -> Vec<Vec<bool>> {
    let total_masks = 2u64.pow(set_mask.iter().filter(|n| **n).count() as u32);
    (0..total_masks)
        .map(|seed| {
            let mask = gen_mask_float(seed, set_mask);
            let one_mask: Vec<bool> = mask.iter().map(|(a, _)| *a).collect();
            let zero_mask: Vec<bool> = mask.iter().map(|(_, a)| *a).collect();
            let number = mask_zeros(number, &zero_mask);
            mask_ones(&number, &one_mask)
        })
        .collect()
}

#[test]
fn test_gen_masks() {
    let mask = mask_from_number(33);
    for seed in 0..4 {
        let mask = gen_mask_float(seed, &mask);
        let one_mask: Vec<bool> = mask.iter().map(|(a, _)| *a).collect();
        let zero_mask: Vec<bool> = mask.iter().map(|(_, a)| *a).collect();
    }
}

#[derive(Debug)]
pub struct Sim2 {
    memory: BTreeMap<u64, u64>,
    mask_float: Vec<bool>,
    mask_one: Vec<bool>,
}
impl Sim2 {
    fn blank() -> Sim2 {
        Sim2 { memory: BTreeMap::new(), mask_float: vec![false; 36], mask_one: vec![false; 36] }
    }
    fn set_masks_from_str(&mut self, mask: &str) {
        self.mask_one = vec![false; 36];
        self.mask_float = vec![false; 36];
        for (i, c) in mask.bytes().rev().enumerate() {
            match c {
                b'X' => self.mask_float[i] = true,
                b'1' => self.mask_one[i] = true,
                b'0' => {}
                _ => panic!(),
            }
        }
    }
    fn set_memory(&mut self, address: u64, value: u64) {
        let address = mask_from_number(address);
        let address = mask_ones(&address, &self.mask_one);
        let addresses = mask_floats(&address, &self.mask_float);
        for address in addresses {
            let address = number_from_mask(&address);
            self.memory.insert(address, value);
        }
    }
    fn simulate(commands: &[Command]) -> Sim2 {
        let mut sim = Sim2::blank();
        for command in commands {
            match command {
                Command::MaskSet(mask) => sim.set_masks_from_str(&mask),
                Command::MemSet(address, value) => sim.set_memory(*address, *value),
            }
        }
        sim
    }
}

pub type Task1 = u64;
pub type Task2 = u64;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let sim = Sim::simulate(&input.commands);
    let sim2 = Sim2::simulate(&input.commands);
    Ok(Output { task1: sim.memory.values().copied().sum(), task2: sim2.memory.values().copied().sum() })
}
