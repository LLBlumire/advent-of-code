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

fn count_bits_at(input: &[u32], bit: u32) -> usize {
    input
        .iter()
        .filter(|number| (*number >> bit) & 1 == 1)
        .count()
}

fn find_gamma(input: &[u32], width: u32) -> u32 {
    (0..width).rev().fold(0, |out, bit| {
        if count_bits_at(input, bit) * 2 >= input.len() {
            (out | 1) << 1
        } else {
            out << 1
        }
    }) >> 1
}

fn find_epsilon_from_gamma(gamma: u32, width: u32) -> u32 {
    (!gamma) & (2_u32.pow(width) - 1)
}

fn task1(input: &ParsedInput) -> Result<u32> {
    let gamma = find_gamma(&input.data, input.width);
    let epsilon = find_epsilon_from_gamma(gamma, input.width);
    Ok(gamma * epsilon)
}

fn filter_bits(source: &[u32], bit: u32, matches: u32) -> Vec<u32> {
    source
        .iter()
        .copied()
        .filter(|&data| (data >> bit) & 1 == matches)
        .collect()
}

fn find_ratings(oxygen: &[u32], co2: &[u32], width: u32, bit: u32) -> Option<(u32, u32)> {
    if bit == 0 || (oxygen.len() == 1 && co2.len() == 1) {
        return Some((*oxygen.first()?, *co2.first()?));
    }
    let bit = bit - 1;

    let oxygen_bit_target = count_bits_at(&oxygen, bit) * 2 >= oxygen.len();
    let oxygen_bit_target = if oxygen_bit_target { 1 } else { 0 };
    let next_oxygen = Some(())
        .filter(|_| oxygen.len() > 1)
        .map(|_| filter_bits(oxygen, bit, oxygen_bit_target));

    let co2_bit_target = count_bits_at(&co2, bit) * 2 < co2.len();
    let co2_bit_target = if co2_bit_target { 1 } else { 0 };
    let next_co2 = Some(())
        .filter(|_| co2.len() > 1)
        .map(|_| filter_bits(co2, bit, co2_bit_target));

    find_ratings(
        next_oxygen.as_ref().map(Vec::as_slice).unwrap_or(oxygen),
        next_co2.as_ref().map(Vec::as_slice).unwrap_or(co2),
        width,
        bit,
    )
}

fn task2(input: &ParsedInput) -> Result<u32> {
    let (oxygen, co2) = find_ratings(&input.data, &input.data, input.width, input.width)
        .ok_or("No ratings found")?;
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
