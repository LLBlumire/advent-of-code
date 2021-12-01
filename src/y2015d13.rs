use aoc::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct ParsedInput<'a> {
    happiness_map: HashMap<(&'a str, &'a str), i64>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, i64, line_ending},
        combinator::{map, value},
        multi::separated_list1,
        sequence::tuple,
    };
    let gain = value(1, tag("gain"));
    let lose = value(-1, tag("lose"));
    let gain_or_lose = alt((gain, lose));
    let record = map(
        tuple((
            alpha1,
            tag(" would "),
            gain_or_lose,
            tag(" "),
            i64,
            tag(" happiness units by sitting next to "),
            alpha1,
            tag("."),
        )),
        |(name_a, _, gain_or_lose, _, value, _, name_b, _)| {
            ((name_a, name_b), gain_or_lose * value)
        },
    );
    let records = separated_list1(line_ending, record);
    let mut parse = map(records, |records| ParsedInput {
        happiness_map: records.into_iter().collect(),
    });
    parse(input)
}

fn get_all_names<'a>(map: &HashMap<(&'a str, &'a str), i64>) -> HashSet<&'a str> {
    map.keys()
        .map(|(a, _)| *a)
        .chain(map.keys().map(|(_, b)| *b))
        .collect()
}

fn task1(input: &ParsedInput) -> Result<i64> {
    let names = get_all_names(&input.happiness_map);
    names
        .iter()
        .permutations(names.len())
        .map(|arangement| {
            arangement
                .iter()
                .cycle()
                .take(arangement.len() + 1)
                .tuple_windows()
                .filter_map(|(a, b)| {
                    Some(input.happiness_map.get(&(a, b))? + input.happiness_map.get(&(b, a))?)
                })
                .sum()
        })
        .max()
        .ok_or_else(|| "No people supplied".into())
}

fn task2(input: &ParsedInput) -> Result<i64> {
    let names = get_all_names(&input.happiness_map);

    names
        .iter()
        .cloned()
        .chain(Some(""))
        .permutations(names.len() + 1)
        .map(|arangement| {
            arangement
                .iter()
                .cycle()
                .take(arangement.len() + 1)
                .tuple_windows()
                .filter_map(|(a, b)| {
                    if a.is_empty() || b.is_empty() {
                        Some(0)
                    } else {
                        Some(input.happiness_map.get(&(a, b))? + input.happiness_map.get(&(b, a))?)
                    }
                })
                .sum()
        })
        .max()
        .ok_or_else(|| "No people supplied".into())
}

#[test]
fn test() {
    let test_in: &str = r#"
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
    "#
    .trim();

    assert_task!(parse, task1, test_in, 330);
}

aoc_main!(parse, task1, task2);
