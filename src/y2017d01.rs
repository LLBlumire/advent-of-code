use aoc::*;
use itertools::Itertools;

struct ParsedInput {
    digits: Vec<u8>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        bytes::complete::take,
        character::complete::u8,
        combinator::{map, map_parser},
        multi::many1,
    };
    let mut parse = map(many1(map_parser(take(1usize), u8)), |digits| ParsedInput {
        digits,
    });
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<u32> {
    Ok(input
        .digits
        .iter()
        .chain(input.digits.first())
        .tuple_windows()
        .filter(|(a, b)| a == b)
        .map(|(a, _)| *a as u32)
        .sum())
}

fn task2(input: &ParsedInput) -> Result<u32> {
    Ok(input
        .digits
        .iter()
        .zip(input.digits.iter().cycle().skip(input.digits.len() / 2))
        .filter(|(a, b)| a == b)
        .map(|(a, _)| *a as u32)
        .sum())
}

#[test]
fn test() {
    assert_task!(parse, task1, "1122", 3);
    assert_task!(parse, task1, "1111", 4);
    assert_task!(parse, task1, "1234", 0);
    assert_task!(parse, task1, "91212129", 9);
    assert_task!(parse, task2, "1212", 6);
    assert_task!(parse, task2, "1221", 0);
    assert_task!(parse, task2, "123425", 4);
    assert_task!(parse, task2, "123123", 12);
    assert_task!(parse, task2, "12131415", 4);
}

aoc_main!(parse, task1, task2);
