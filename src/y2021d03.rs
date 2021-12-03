use aoc::*;

#[derive(Debug)]
struct ParsedInput {
    width: u32,
    data: Vec<u32>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{digit1, line_ending},
        combinator::{map, map_res},
        multi::separated_list1,
    };
    let width = input
        .lines()
        .next()
        .map(|line| line.len() as u32)
        .unwrap_or_default();
    let binary_number = map_res(digit1, |num| u32::from_str_radix(num, 2));
    let numbers = separated_list1(line_ending, binary_number);
    let mut parse = map(numbers, |data| ParsedInput { width, data });
    parse(input)
}

fn find_target(input: &[u32], width: u32, predicate: impl Fn(usize) -> bool) -> u32 {
    (0..width).rev().fold(0, |out, bit| {
        if predicate(
            input
                .iter()
                .filter(|number| ({ (*number >> bit) & (2_u32.pow(1) - 1) }) == 1)
                .count()
                * 2,
        ) {
            (out | 1) << 1
        } else {
            out << 1
        }
    }) >> 1
}

fn find_gamma(input: &[u32], width: u32) -> u32 {
    let len = input.len();
    find_target(input, width, |n| n >= len)
}

fn find_epsilon(input: &[u32], width: u32) -> u32 {
    let len = input.len();
    find_target(input, width, |n| n <= len)
}

fn task1(input: &ParsedInput) -> Result<u32> {
    let gamma = find_gamma(&input.data, input.width);
    let epsilon = find_epsilon(&input.data, input.width);
    Ok(gamma * epsilon)
}

fn find_rating_with_target(
    input: &[u32],
    width: u32,
    target: fn(&[u32], u32) -> u32,
    bit_care: u32,
) -> Option<u32> {
    if input.len() <= 1 {
        return input.first().copied();
    }
    let bit_care = bit_care - 1;
    let target_value = target(input, width);
    let next_inputs = input
        .iter()
        .cloned()
        .filter(|data| (target_value >> bit_care) & 1 == (data >> bit_care) & 1)
        .collect::<Vec<_>>();
    find_rating_with_target(&next_inputs, width, target, bit_care)
}

fn find_oxygen_rating(input: &[u32], width: u32, bit_care: u32) -> Option<u32> {
    find_rating_with_target(input, width, find_gamma, bit_care)
}

fn find_co2_rating(input: &[u32], width: u32, bit_care: u32) -> Option<u32> {
    find_rating_with_target(input, width, find_epsilon, bit_care)
}

fn task2(input: &ParsedInput) -> Result<u32> {
    let oxygen = find_oxygen_rating(&input.data, input.width, input.width)
        .ok_or("Cannot find oxygen rating")?;
    let co2 = find_co2_rating(&input.data, input.width, input.width - 1)
        .ok_or("Cannot find co2 rating")?;
    Ok(oxygen * co2)
}

#[test]
fn test() {
    let test_input = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
    "#
    .trim();

    assert_task!(parse, task1, test_input, 198);
    assert_task!(parse, task2, test_input, 230);
}

aoc_main!(parse, task1, task2);
