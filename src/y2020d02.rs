use aoc::*;
use std::str::FromStr;

struct ParsedInput<'a> {
    passwords: Vec<Password<'a>>,
}
struct Password<'a> {
    password: &'a str,
    character: char,
    param1: usize,
    param2: usize,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, char, digit1, line_ending, satisfy},
        combinator::{map, map_res},
        multi::separated_list1,
        sequence::{separated_pair, tuple},
    };

    let usize = |i| map_res(digit1, <usize as FromStr>::from_str)(i);
    let params = separated_pair(usize, char('-'), usize);
    let record = tuple((
        params,
        char(' '),
        satisfy(char::is_alphabetic),
        tag(": "),
        alpha1,
    ));
    let password = map(record, |((param1, param2), _, character, _, password)| {
        Password {
            param1,
            param2,
            character,
            password,
        }
    });
    let passwords = separated_list1(line_ending, password);
    let mut parse = map(passwords, |passwords| ParsedInput { passwords });
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<usize> {
    Ok(input
        .passwords
        .iter()
        .filter(|password| {
            let count = password
                .password
                .chars()
                .filter(|&c| c == password.character)
                .count();
            password.param1 <= count && count <= password.param2
        })
        .count())
}

fn task2(input: &ParsedInput) -> Result<usize> {
    Ok(input
        .passwords
        .iter()
        .filter(|password| {
            let mut chars = password.password.chars();
            let pos1 = chars.nth(password.param1 - 1) == Some(password.character);
            let pos2 = chars.nth(password.param2 - 1 - password.param1) == Some(password.character);
            pos1 ^ pos2
        })
        .count())
}

#[test]
fn test() {
    let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    assert_task!(parse, task1, input, 2);
    assert_task!(parse, task2, input, 1);
}

aoc_main!(parse, task1, task2);
