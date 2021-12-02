use std::collections::HashMap;

use aoc::*;
use itertools::Itertools;

struct ParsedInput {
    jolts: Vec<i64>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{i64, line_ending},
        combinator::map,
        multi::separated_list1,
    };
    let mut parse = map(separated_list1(line_ending, i64), |mut jolts| {
        jolts.push(0);
        jolts.sort_unstable();
        if let Some(last) = jolts.last().copied() {
            jolts.push(last + 3)
        }
        ParsedInput { jolts }
    });
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<i64> {
    Ok(input
        .jolts
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .fold(Ok((0, 0, 0)), |acc, item| {
            if let Ok((one, two, three)) = acc {
                Ok(match item {
                    1 => (one + 1, two, three),
                    2 => (one, two + 1, three),
                    3 => (one, two, three + 1),
                    _ => return Err("unexpected skip of four jolts"),
                })
            } else {
                acc
            }
        })
        .map(|(one, _, three)| one * three)?)
}

fn get_num_combinations<'a>(chain: &'a [i64], known: &mut HashMap<&'a [i64], i64>) -> Result<i64> {
    if let Some(cached) = known.get(chain) {
        return Ok(*cached);
    }
    if chain.len() == 1 {
        known.insert(chain, 1);
        return Ok(1);
    }
    let mut legal_chains = Vec::new();
    if let (&[source], tail) = chain.split_at(1) {
        for (i, &target) in tail.iter().enumerate() {
            if target <= source + 3 {
                legal_chains.push(&tail[i..])
            }
        }
    } else {
        return Err("invalid chain".into());
    }
    let sum = legal_chains
        .into_iter()
        .map(|i| get_num_combinations(i, known))
        .collect::<Result<Vec<i64>>>()?
        .into_iter()
        .sum();
    known.insert(chain, sum);
    Ok(sum)
}

fn task2(input: &ParsedInput) -> Result<i64> {
    get_num_combinations(&input.jolts, &mut HashMap::new())
}

#[test]
fn test() {}

aoc_main!(parse, task1, task2);
