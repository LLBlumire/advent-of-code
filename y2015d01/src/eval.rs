use crate::*;

#[derive(Debug)]
pub struct Output { 
    final_floor: i32,
    first_basement: usize
}
impl TryFrom<ParsedInput> for Output {
    type Error = Error;
    fn try_from(parsed_input: ParsedInput) -> Result<Output> {
        Ok(Output { 
            final_floor: parsed_input.elevator_signals.iter().sum(),
            first_basement: parsed_input.elevator_signals.iter().enumerate().scan(0, |state, (i, item)| {
                *state += item;
                Some((i, *state))
            }).skip_while(|(_, state)| *state >= 0).map(|(i, _)| i + 1).next().ok_or(Error::None)?
        })
    }
}
