use crate::*;
pub struct ParsedInput {
    pub values: Vec<i32>,
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
    let (input, values) = parsers::number_list(input)?;
    Ok((input, ParsedInput { values }))
}
