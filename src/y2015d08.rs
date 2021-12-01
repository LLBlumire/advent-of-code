use aoc::*;
#[derive(Debug)]
struct Record<'a> {
    original: &'a str,
    processed: String,
}
#[derive(Debug)]
struct ParsedInput<'a> {
    inputs: Vec<Record<'a>>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        branch::alt,
        bytes::complete::{escaped_transform, take},
        character::complete::{alpha1, char, line_ending},
        combinator::{map, map_res, value},
        multi::separated_list1,
        sequence::{delimited, preceded},
    };

    let hex_seq = map_res(preceded(char('x'), take(2usize)), |i| {
        u32::from_str_radix(i, 16)
            .map(char::from_u32)
            .map(Option::unwrap_or_default)
    });
    let slash_seq = value('\\', char('\\'));
    let quote_seq = value('\"', char('\"'));
    let seq = alt((slash_seq, quote_seq, hex_seq));
    let escaped = escaped_transform(alpha1, '\\', seq);
    let quoted = delimited(char('\"'), escaped, char('\"'));
    let inputs = separated_list1(line_ending, quoted);
    let mut parse = map(inputs, |inputs| ParsedInput {
        inputs: inputs
            .into_iter()
            .zip(input.lines())
            .map(|(processed, original)| Record {
                processed,
                original,
            })
            .collect(),
    });
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<usize> {
    Ok(input
        .inputs
        .iter()
        .map(|record| record.original.chars().count())
        .sum::<usize>()
        - input
            .inputs
            .iter()
            .map(|record| record.processed.chars().count())
            .sum::<usize>())
}

fn task2(input: &ParsedInput) -> Result<usize> {
    fn encode(input: &str) -> String {
        format!(
            "\"{}\"",
            input
                .chars()
                .map(|c| match c {
                    '\"' => "\\\"".to_string(),
                    '\\' => "\\\\".to_string(),
                    a => a.to_string(),
                })
                .collect::<String>()
        )
    }
    Ok(input
        .inputs
        .iter()
        .map(|record| encode(record.original).chars().count())
        .sum::<usize>()
        - input
            .inputs
            .iter()
            .map(|record| record.original.chars().count())
            .sum::<usize>())
}

#[test]
fn test() {}

aoc_main!(parse, task1, task2);
