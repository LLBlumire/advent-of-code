use aoc::*;
use itertools::{Itertools, MinMaxResult};

struct ParsedInput {
    code: Vec<i32>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{i32, line_ending},
        combinator::map,
        multi::separated_list1,
    };
    let mut parse = map(separated_list1(line_ending, i32), |code| ParsedInput {
        code,
    });
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<i32> {
    Ok(input
        .code
        .windows(26)
        .find_map(|window| {
            let last = *window.last()?;
            if window
                .iter()
                .tuple_combinations()
                .any(|(&a, &b)| a + b == last)
            {
                None
            } else {
                Some(last)
            }
        })
        .ok_or("No invalid number")?)
}

fn task2(input: &ParsedInput, task1: i32) -> Result<i32> {
    Ok((2..)
        .map(|size| input.code.windows(size))
        .flat_map(|windows| windows.filter(|window| window.iter().sum::<i32>() == task1))
        .find_map(|legal_sequence| {
            if let MinMaxResult::MinMax(low, high) = legal_sequence.iter().minmax() {
                Some(low + high)
            } else {
                None
            }
        })
        .ok_or("No valid sequence")?)
}

#[test]
fn test() {}

aoc_main!(parse, task1 -> task2);
