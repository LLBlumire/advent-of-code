use aoc::*;
use ndarray::{s, Array2, ArrayBase, Dim, ViewRepr};

struct ParsedInput {
    commands: Vec<Command>,
}

#[derive(Copy, Clone)]
enum Operation {
    TurnOn,
    Toggle,
    TurnOff,
}

struct Command {
    operation: Operation,
    from: (usize, usize),
    to: (usize, usize),
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, line_ending, u32},
        combinator::{map, value},
        multi::separated_list1,
        sequence::{separated_pair, tuple},
    };
    let turn_on = value(Operation::TurnOn, tag("turn on "));
    let toggle = value(Operation::Toggle, tag("toggle "));
    let turn_off = value(Operation::TurnOff, tag("turn off "));
    let operation = alt((turn_on, toggle, turn_off));
    let coord_u32 = |i| separated_pair(u32, char(','), u32)(i);
    let coord_usize = |i| map(coord_u32, |(a, b)| (a as usize, b as usize))(i);
    let from_to = separated_pair(coord_usize, tag(" through "), coord_usize);
    let command_raw = tuple((operation, from_to));
    let command = map(command_raw, |(operation, (from, to))| Command {
        operation,
        from,
        to,
    });
    let commands = separated_list1(line_ending, command);
    let mut parsed = map(commands, |commands| ParsedInput { commands });
    parsed(input)
}

struct LightSimulator {
    lights: Array2<u32>,
}
impl LightSimulator {
    fn new() -> LightSimulator {
        LightSimulator {
            lights: Array2::zeros((1000, 1000)),
        }
    }
    fn target_slice_mut(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
    ) -> ArrayBase<ViewRepr<&mut u32>, Dim<[usize; 2]>> {
        self.lights.slice_mut(s![from.0..=to.0, from.1..=to.1])
    }
    fn simulate_v1<'a>(&mut self, commands: impl IntoIterator<Item = &'a Command>) -> &mut Self {
        for command in commands {
            let mut target = self.target_slice_mut(command.from, command.to);
            match command.operation {
                Operation::TurnOn => target.fill(1),
                Operation::Toggle => target.mapv_inplace(|i| i ^ 1),
                Operation::TurnOff => target.fill(0),
            }
        }
        self
    }
    fn simulate_v2<'a>(&mut self, commands: impl IntoIterator<Item = &'a Command>) -> &mut Self {
        for command in commands {
            let mut target = self.target_slice_mut(command.from, command.to);
            match command.operation {
                Operation::TurnOn => target.mapv_inplace(|i| i + 1),
                Operation::Toggle => target.mapv_inplace(|i| i + 2),
                Operation::TurnOff => target.mapv_inplace(|i| i.saturating_sub(1)),
            }
        }
        self
    }
    fn total_brightness(&self) -> u32 {
        self.lights.sum()
    }
}

fn task1(input: &ParsedInput) -> Result<u32> {
    Ok(LightSimulator::new()
        .simulate_v1(&input.commands)
        .total_brightness())
}

fn task2(input: &ParsedInput) -> Result<u32> {
    Ok(LightSimulator::new()
        .simulate_v2(&input.commands)
        .total_brightness())
}

#[test]
fn test() {
    assert_task!(parse, task1, "turn on 0,0 through 999,999", 1000000);
    assert_task!(parse, task1, "toggle 0,0 through 999,0", 1000);
    assert_task!(
        parse,
        task1,
        "turn on 0,0 through 999,999\nturn off 499,499 through 500,500",
        999996
    );
    assert_task!(parse, task2, "turn on 0,0 through 0,0", 1);
    assert_task!(parse, task2, "toggle 0,0 through 999,999", 2000000);
}

aoc_main!(parse, task1, task2);
