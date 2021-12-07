use std::collections::BTreeMap;

use aoc::*;

struct ParsedInput {
    fish_timings: BTreeMap<u32, usize>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{char, u32},
        combinator::map,
        multi::separated_list1,
        Parser,
    };
    let mut parser = map(separated_list1(char(','), u32), |fish_timings| {
        ParsedInput {
            fish_timings: fish_timings
                .iter()
                .fold(BTreeMap::new(), |mut timings, &timing| {
                    *timings.entry(timing).or_insert(0) += 1;
                    timings
                }),
        }
    });
    parser.parse(input)
}

fn task1(input: &ParsedInput) -> Result<usize> {
    let mut fish_timings = input.fish_timings.clone();
    for _ in 0..80 {
        fish_timings =
            fish_timings
                .iter()
                .fold(BTreeMap::new(), |mut next_timings, (timing, count)| {
                    if let Some(next) = timing.checked_sub(1) {
                        *next_timings.entry(next).or_insert(0) += count;
                    } else {
                        *next_timings.entry(6).or_insert(0) += count;
                        *next_timings.entry(8).or_insert(0) += count;
                    }
                    next_timings
                });
    }
    Ok(fish_timings.values().sum())
}

fn task2(input: &ParsedInput) -> Result<usize> {
    let mut fish_timings = input.fish_timings.clone();
    for _ in 0..256 {
        fish_timings =
            fish_timings
                .iter()
                .fold(BTreeMap::new(), |mut next_timings, (timing, count)| {
                    if let Some(next) = timing.checked_sub(1) {
                        *next_timings.entry(next).or_insert(0) += count;
                    } else {
                        *next_timings.entry(6).or_insert(0) += count;
                        *next_timings.entry(8).or_insert(0) += count;
                    }
                    next_timings
                });
    }
    Ok(fish_timings.values().sum())
}

#[test]
fn test() {
    assert_task!(parse, task1, "3,4,3,1,2", 5934);
    assert_task!(parse, task2, "3,4,3,1,2", 26984457539_usize);
}

aoc_main!(parse, task1, task2);
