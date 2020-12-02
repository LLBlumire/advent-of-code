use crate::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParsedInput {
    records: Vec<i32>,
}

pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let number = map_res(digit1, FromStr::from_str);
    let numbers = separated_list0(line_ending, number);
    let mut parsed = map(numbers, |records| ParsedInput { records });
    Ok(parsed(input)?)
}

pub type Task1 = i32;
pub type Task2 = i32;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input
            .records
            .iter()
            .combinations(2)
            .filter(|x| x[0] + x[1] == 2020)
            .map(|x| x[0] * x[1])
            .next()
            .ok_or(Error::None)?,
        task2: input
            .records
            .iter()
            .combinations(3)
            .filter(|x| x[0] + x[1] + x[2] == 2020)
            .map(|x| x[0] * x[1] * x[2])
            .next()
            .ok_or(Error::None)?,
    })
}
