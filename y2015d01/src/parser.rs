use crate::*;
use nom::{character::complete::*, multi::*, branch::*, combinator::*};

pub struct ParsedInput {
    pub elevator_signals: Vec<i32>
}
impl TryFrom<&str> for ParsedInput {
    type Error = Error;
    fn try_from(input: &str) -> Result<ParsedInput> {
        Ok(parse_internal(input)
            .finish()
            .map(|(_, p)| p)
            .map_err(|e| nom::error::Error::new(e.input.to_string(), e.code))?)
    }
}

pub fn parse_internal(input: &str) -> IResult<&str, ParsedInput> {
    let (input, elevator_signals) = elevator_signals(input)?;
    Ok((input, ParsedInput { elevator_signals }))
}

fn elevator_up(input: &str) -> IResult<&str, i32> {
    map(char('('), |_| 1)(input)
}
fn elevator_down(input: &str) -> IResult<&str, i32> {
    map(char(')'), |_| -1)(input)
}
fn elevator_signal(input: &str) -> IResult<&str, i32> {
    alt((elevator_up, elevator_down))(input)
}
fn elevator_signals(input: &str) -> IResult<&str, Vec<i32>> {
    many0(elevator_signal)(input)
}