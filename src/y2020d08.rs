use aoc::*;

#[derive(Debug)]
struct ParsedInput {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    operation: Operation,
    argument: i64,
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Accumulator,
    Jump,
    NoOp,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, i64, line_ending},
        combinator::{map, value},
        multi::separated_list1,
        sequence::separated_pair,
    };
    let acc = value(Operation::Accumulator, tag("acc"));
    let jmp = value(Operation::Jump, tag("jmp"));
    let nop = value(Operation::NoOp, tag("nop"));
    let operation = alt((acc, jmp, nop));
    let instruction = map(
        separated_pair(operation, char(' '), i64),
        |(operation, argument)| Instruction {
            operation,
            argument,
        },
    );
    let instructions = separated_list1(line_ending, instruction);
    let mut parse = map(instructions, |instructions| ParsedInput { instructions });
    parse(input)
}

#[derive(Default)]
struct Computer {
    accumulator: i64,
    instruction_pointer: i64,
    trace: Vec<i64>,
}
impl Computer {
    fn eval_without_loop(instructions: &[Instruction]) -> std::result::Result<i64, i64> {
        let mut computer = Computer::default();
        while let Some(instruction) = instructions.get(computer.instruction_pointer as usize) {
            if computer.detect_loop() {
                return Err(computer.accumulator);
            }
            computer.step(instruction);
        }
        Ok(computer.accumulator)
    }
    fn step(&mut self, instruction: &Instruction) {
        self.trace.push(self.instruction_pointer);
        match instruction.operation {
            Operation::Accumulator => self.accumulator += instruction.argument,
            Operation::Jump => self.instruction_pointer += instruction.argument - 1,
            Operation::NoOp => {}
        }
        self.instruction_pointer += 1;
    }
    fn detect_loop(&self) -> bool {
        self.trace.contains(&self.instruction_pointer)
    }
}

fn task1(input: &ParsedInput) -> Result<i64> {
    Ok(Computer::eval_without_loop(&input.instructions)
        .err()
        .ok_or("unexpected normal termination")?)
}

fn task2(input: &ParsedInput) -> Result<i64> {
    Ok(input
        .instructions
        .iter()
        .enumerate()
        .filter_map(|(i, instruction)| {
            let mut candidate = input.instructions.clone();
            candidate[i] = Instruction {
                operation: match instruction.operation {
                    Operation::Accumulator => return None,
                    Operation::Jump => Operation::NoOp,
                    Operation::NoOp => Operation::Jump,
                },
                argument: instruction.argument,
            };
            Some(candidate)
        })
        .filter_map(|instructions| Computer::eval_without_loop(&instructions).ok())
        .next()
        .ok_or("No solution available")?)
}

#[test]
fn test() {}

aoc_main!(parse, task1, task2);
