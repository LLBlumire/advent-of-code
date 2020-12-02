use crate::*;
use std::str::FromStr;
#[derive(Debug)]
pub struct ParsedInput {
    passwords: Vec<Password>,
}

#[derive(Debug)]
struct Password {
    password: String,
    character: char,
    param1: usize,
    param2: usize,
}
impl Password {
    fn is_sled_valid(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.character).count();
        self.param1 <= count && count <= self.param2
    }
    fn is_toboggan_valid(&self) -> bool {
        let mut chars = self.password.chars();
        let pos1 = chars.nth(self.param1 - 1) == Some(self.character);
        let pos2 = chars.nth(self.param2 - 1 - self.param1) == Some(self.character);
        pos1 ^ pos2
    }
}

pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let number = |i| map_res(digit1, FromStr::from_str)(i);
    let params = separated_pair(number, char('-'), number);
    let record = tuple((params, char(' '), anychar, tag(": "), map(alpha0, ToString::to_string)));
    let parsed_record =
        map(record, |((param1, param2), _, character, _, password)| Password { password, character, param1, param2 });
    let records = separated_list0(line_ending, parsed_record);
    let mut parsed = map(records, |passwords| ParsedInput { passwords });

    Ok(parsed(input)?)
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input.passwords.iter().filter(|p| p.is_sled_valid()).count(),
        task2: input.passwords.iter().filter(|p| p.is_toboggan_valid()).count(),
    })
}
