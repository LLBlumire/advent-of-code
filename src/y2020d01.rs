use aoc::*;
use itertools::Itertools;

struct ParsedInput {
    records: Vec<i32>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{i32, line_ending},
        combinator::map,
        multi::separated_list1,
    };
    let records = separated_list1(line_ending, i32);
    let mut parse = map(records, |records| ParsedInput { records });
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<i32> {
    input
        .records
        .iter()
        .tuple_combinations()
        .filter(|&(a, b)| a + b == 2020)
        .map(|(a, b)| a * b)
        .next()
        .ok_or_else(|| "Expected input".into())
}

fn task2(input: &ParsedInput) -> Result<i32> {
    input
        .records
        .iter()
        .tuple_combinations()
        .filter(|&(a, b, c)| a + b + c == 2020)
        .map(|(a, b, c)| a * b * c)
        .next()
        .ok_or_else(|| "Expected input".into())
}

#[test]
fn test() {
    let input = "1721\n979\n366\n299\n675\n1456";
    assert_task!(parse, task1, input, 514579);
    assert_task!(parse, task2, input, 241861950);
}

aoc_main!(parse, task1, task2);
