use crate::*;

#[derive(Debug)]
pub struct ParsedInput {
    elevator_signals: Vec<i32>,
}

pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let up = map(char('('), |_| 1);
    let down = map(char(')'), |_| -1);
    let signal = alt((up, down));
    let signals = many0(signal);
    let mut parsed = map(signals, |elevator_signals| ParsedInput { elevator_signals });
    Ok(parsed(input)?)
}

pub type Task1 = i32;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input.elevator_signals.iter().sum(),
        task2: input
            .elevator_signals
            .iter()
            .enumerate()
            .scan(0, |state, (i, item)| {
                *state += item;
                Some((i, *state))
            })
            .skip_while(|(_, state)| *state >= 0)
            .map(|(i, _)| i + 1)
            .next()
            .ok_or(Error::None)?,
    })
}
