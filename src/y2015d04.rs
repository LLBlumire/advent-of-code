use aoc::*;

struct ParsedInput<'a> {
    key: &'a str,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{character::complete::alpha1, combinator::map};
    let mut parse = map(alpha1, |key| ParsedInput { key });
    parse(input)
}

fn mine(key: &str, zeroes: usize) -> Result<usize> {
    let lead = format!("{:0>pad$}", "", pad = zeroes);
    for i in 0.. {
        if format!("{:x}", md5::compute(&format!("{}{}", key, i)))[..zeroes] == lead {
            return Ok(i);
        }
    }
    Err("No such coin exists".into())
}

fn task1(input: &ParsedInput) -> Result<usize> {
    mine(input.key, 5)
}

fn task2(input: &ParsedInput) -> Result<usize> {
    mine(input.key, 6)
}

#[test]
fn test() {
    assert_task!(parse, task1, "abcdef", 609043);
    assert_task!(parse, task1, "pqrstuv", 1048970);
}

aoc_main!(parse, task1, task2);
