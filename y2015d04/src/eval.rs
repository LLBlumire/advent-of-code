use crate::*;

#[derive(Debug)]
pub struct ParsedInput {
    key: String,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    Ok(("", ParsedInput { key: input.to_string() }))
}

fn mine(pk: &str, zeroes: usize) -> usize {
    let lead = &format!("{:0>pad$}", "", pad = zeroes)[..];
    for i in 0.. {
        if &format!("{:x}", md5::compute(&format!("{}{}", pk, i)))[..zeroes] == lead {
            return i;
        }
    }
    panic!()
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output { task1: mine(&input.key, 5), task2: mine(&input.key, 6) })
}
