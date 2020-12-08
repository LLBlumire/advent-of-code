use crate::*;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Accumulator,
    Jump,
    NoOperation,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    operation: Operation,
    argument: i64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParsedInput {
    instructions: Vec<Instruction>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let acc = map(tag("acc"), |_| Operation::Accumulator);
    let jmp = map(tag("jmp"), |_| Operation::Jump);
    let nop = map(tag("nop"), |_| Operation::NoOperation);
    let operation = alt((acc, jmp, nop));
    let pos = map(tag("+"), |_| 1);
    let neg = map(tag("-"), |_| -1);
    let sgn = alt((pos, neg));
    let num = map_res(digit1, FromStr::from_str);
    let signed_number = map(tuple((sgn, num)), |(s, n): (i64, i64)| s * n);
    let instruction = map(separated_pair(operation, tag(" "), signed_number), |(operation, argument)| Instruction {
        operation,
        argument,
    });
    let mut parsed = map(separated_list1(line_ending, instruction), |instructions| ParsedInput { instructions });

    Ok(parsed(input)?)
}

#[derive(Debug, Clone, Default)]
pub struct Computer {
    accumulator: i64,
    instruction_pointer: i64,
    trace: Vec<i64>,
}
impl Computer {
    fn eval_check_loop(instructions: &[Instruction]) -> std::result::Result<i64, i64> {
        let mut computer = Computer::default();
        while let Some(instruction) = instructions.get(computer.instruction_pointer as usize) {
            if computer.detect_loop() {
                return Err(computer.accumulator);
            }
            computer.step(instruction, true);
        }
        Ok(computer.accumulator)
    }
    fn step(&mut self, instruction: &Instruction, trace: bool) {
        if trace {
            self.trace.push(self.instruction_pointer)
        }
        let Instruction { operation, argument } = instruction;
        match operation {
            Operation::Accumulator => self.accumulator += argument,
            Operation::Jump => self.instruction_pointer += argument - 1,
            Operation::NoOperation => {}
        }
        self.instruction_pointer += 1;
    }
    fn detect_loop(&self) -> bool {
        self.trace.contains(&self.instruction_pointer)
    }
}

fn generate_fix_candidates(instructions: &[Instruction]) -> Vec<Vec<Instruction>> {
    let mut master_list: Vec<Vec<Instruction>> = Vec::new();
    for (i, instruction) in instructions.iter().enumerate() {
        let new_op = match instruction.operation {
            Operation::Jump => Operation::NoOperation,
            Operation::NoOperation => Operation::Jump,
            other => continue,
        };
        let mut candidate: Vec<Instruction> = Vec::from(instructions);
        candidate[i] = Instruction { operation: new_op, argument: instruction.argument };
        master_list.push(candidate)
    }
    master_list.push(Vec::from(instructions));
    master_list
}

pub type Task1 = i64;
pub type Task2 = i64;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: Computer::eval_check_loop(&input.instructions).unwrap_err(),
        task2: generate_fix_candidates(&input.instructions)
            .into_iter()
            .filter_map(|instructions| Computer::eval_check_loop(&instructions).ok())
            .next()
            .unwrap(),
    })
}
