use crate::*;
use arr::{s,
          Array2};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Operation {
    TurnOn,
    Toggle,
    TurnOff,
}

#[derive(Debug)]
struct Command {
    operation: Operation,
    from: (usize, usize),
    to: (usize, usize),
}

#[derive(Debug)]
pub struct ParsedInput {
    commands: Vec<Command>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let turn_on = map(tag("turn on "), |_| Operation::TurnOn);
    let toggle = map(tag("toggle "), |_| Operation::Toggle);
    let turn_off = map(tag("turn off "), |_| Operation::TurnOff);
    let operation = alt((turn_on, toggle, turn_off));
    let number = |i| map_res(digit1, FromStr::from_str)(i);
    let coord = |i| separated_pair(number, char(','), number)(i);
    let from_to = separated_pair(coord, tag(" through "), coord);
    let command = map(tuple((operation, from_to)), |(operation, (from, to))| Command { operation, from, to });
    let mut parsed = map(separated_list1(line_ending, command), |commands| ParsedInput { commands });
    Ok(parsed(input)?)
}

#[derive(Debug)]
pub struct LightSimulator {
    lights: Array2<i32>,
}
impl LightSimulator {
    fn new() -> LightSimulator {
        LightSimulator { lights: Array2::zeros((1000, 1000)) }
    }

    fn run_command_v1(&mut self, command: &Command) {
        let mut target = self.lights.slice_mut(s![command.from.0..=command.to.0, command.from.1..=command.to.1]);
        match command.operation {
            Operation::TurnOn => target.fill(1),
            Operation::Toggle => target ^= 1,
            Operation::TurnOff => target.fill(0),
        }
    }

    fn simulate_v1(commands: &[Command]) -> LightSimulator {
        let mut sim = LightSimulator::new();
        for command in commands {
            sim.run_command_v1(command)
        }
        sim
    }

    fn run_command_v2(&mut self, command: &Command) {
        let mut target = self.lights.slice_mut(s![command.from.0..=command.to.0, command.from.1..=command.to.1]);
        match command.operation {
            Operation::TurnOn => target += 1,
            Operation::Toggle => target += 2,
            Operation::TurnOff => target.map_inplace(|n| *n = (*n - 1).max(0)),
        }
    }

    fn simulate_v2(commands: &[Command]) -> LightSimulator {
        let mut sim = LightSimulator::new();
        for command in commands {
            sim.run_command_v2(command)
        }
        sim
    }
}

pub type Task1 = i32;
pub type Task2 = i32;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: LightSimulator::simulate_v1(&input.commands).lights.sum(),
        task2: LightSimulator::simulate_v2(&input.commands).lights.sum(),
    })
}
