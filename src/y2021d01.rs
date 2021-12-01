use aoc::*;
use itertools::Itertools;

struct ParsedInput {
    depths: Vec<i32>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{i32, line_ending},
        combinator::map,
        multi::separated_list1,
    };
    let numbers = separated_list1(line_ending, i32);
    let mut parse = map(numbers, |depths| ParsedInput { depths });
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<usize> {
    Ok(input
        .depths
        .iter()
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count())
}

fn task2(input: &ParsedInput) -> Result<usize> {
    Ok(input
        .depths
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count())
}

#[test]
fn test() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    assert_task!(parse, task1, input, 7);
    assert_task!(parse, task2, input, 5);
}

aoc_main!(parse, task1, task2);
