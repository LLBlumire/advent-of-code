use aoc::*;

#[derive(Debug)]
struct ParsedInput {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Instruction {
    kind: InstructionKind,
    magnitude: i32,
}

#[derive(Copy, Clone, Debug)]
enum InstructionKind {
    Forward,
    Up,
    Down,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, i32, line_ending},
        combinator::{map, value},
        multi::separated_list1,
        sequence::separated_pair,
    };
    let forward = value(InstructionKind::Forward, tag("forward"));
    let up = value(InstructionKind::Up, tag("up"));
    let down = value(InstructionKind::Down, tag("down"));
    let instruction_kind = alt((forward, up, down));
    let instruction = map(
        separated_pair(instruction_kind, char(' '), i32),
        |(kind, magnitude)| Instruction { kind, magnitude },
    );
    let instructions = separated_list1(line_ending, instruction);
    let mut parse = map(instructions, |instructions| ParsedInput { instructions });
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<i32> {
    let (forward, depth) = input
        .instructions
        .iter()
        .fold((0, 0), |(forward, depth), item| match item.kind {
            InstructionKind::Forward => (forward + item.magnitude, depth),
            InstructionKind::Up => (forward, depth - item.magnitude),
            InstructionKind::Down => (forward, depth + item.magnitude),
        });
    Ok(forward * depth)
}

fn task2(input: &ParsedInput) -> Result<i32> {
    let (forward, depth, _) =
        input
            .instructions
            .iter()
            .fold((0, 0, 0), |(forward, depth, aim), item| match item.kind {
                InstructionKind::Forward => {
                    (forward + item.magnitude, depth + aim * item.magnitude, aim)
                }
                InstructionKind::Up => (forward, depth, aim - item.magnitude),
                InstructionKind::Down => (forward, depth, aim + item.magnitude),
            });
    Ok(forward * depth)
}

#[test]
fn test() {
    let test = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
    assert_task!(parse, task1, test, 150);
    assert_task!(parse, task2, test, 900);
}

aoc_main!(parse, task1, task2);
