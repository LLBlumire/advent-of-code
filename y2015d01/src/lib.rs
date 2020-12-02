pub use common::*;

mod eval;
pub use eval::compute;
pub use eval::parse;
pub use eval::ParsedInput;
pub use eval::Task1;
pub use eval::Task2;

impl TryFrom<&str> for ParsedInput {
    type Error = Error;
    fn try_from(input: &str) -> Result<ParsedInput> {
        Ok(parse(input)
            .finish()
            .map(|(_, p)| p)
            .map_err(|e| nom::error::Error::new(e.input.to_string(), e.code))?)
    }
}

#[derive(Debug)]
pub struct Output {
    pub task1: Task1,
    pub task2: Task2,
}
impl TryFrom<ParsedInput> for Output {
    type Error = Error;
    fn try_from(parsed_input: ParsedInput) -> Result<Output> {
        Ok(compute(parsed_input)?)
    }
}
