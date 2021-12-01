use aoc::*;

struct ParsedInput {
    signals: Vec<i32>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        branch::alt,
        character::complete::char,
        combinator::{map, value},
        multi::many1,
    };
    let up = value(1, char('('));
    let down = value(-1, char(')'));
    let signal = alt((up, down));
    let signals = many1(signal);
    let mut parse = map(signals, |signals| ParsedInput { signals });
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<i32> {
    Ok(input.signals.iter().sum())
}

fn task2(input: &ParsedInput) -> Result<usize> {
    Ok(input
        .signals
        .iter()
        .enumerate()
        .scan(0, |state, (i, item)| {
            *state += item;
            Some((i, *state))
        })
        .skip_while(|(_, state)| *state >= 0)
        .map(|(i, _)| i + 1)
        .next()
        .ok_or("Missing inputs")?)
}

#[test]
fn test() {
    assert_task!(parse, task1, "(())", 0);
    assert_task!(parse, task1, "()()", 0);
    assert_task!(parse, task1, "(((", 3);
    assert_task!(parse, task1, "(()(()(", 3);
    assert_task!(parse, task1, "))(((((", 3);
    assert_task!(parse, task1, "())", -1);
    assert_task!(parse, task1, "))(", -1);
    assert_task!(parse, task1, ")))", -3);
    assert_task!(parse, task1, ")())())", -3);
    assert_task!(parse, task2, ")", 1);
    assert_task!(parse, task2, "()())", 5);
}

aoc_main!(parse, task1, task2);
