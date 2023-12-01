use std::collections::BTreeSet;

use aoc::*;
use itertools::Itertools;

#[derive(Debug)]
struct ParsedInput<'input> {
    elves: Vec<Bag<'input>>,
}

#[derive(Debug)]
struct Bag<'input> {
    left: &'input str,
    right: &'input str,
}

fn item_to_priority(item: char) -> i32 {
    if item.is_ascii_uppercase() {
        (item as i32) - 38
    } else if item.is_ascii_lowercase() {
        (item as i32) - 96
    } else {
        panic!("Unexpected input!")
    }
}

fn parse<'input>(input: &'input str) -> ParseResult<ParsedInput<'input>> {
    use nom::{
        character::complete::{alpha1, line_ending},
        multi::separated_list1,
        Parser,
    };
    let bag = alpha1.map(|content: &str| {
        let (left, right) = content.split_at(content.len() / 2);
        Bag { left, right }
    });
    let mut parser = separated_list1(line_ending, bag).map(|elves| ParsedInput { elves });
    parser.parse(input)
}

fn task1(input: &ParsedInput) -> Result<i32> {
    Ok(input
        .elves
        .iter()
        .map(|bag| {
            let left: BTreeSet<char> = bag.left.chars().collect();
            let right: BTreeSet<char> = bag.right.chars().collect();
            let in_both: &char = left.intersection(&right).next().unwrap();
            item_to_priority(*in_both)
        })
        .sum())
}

fn task2(input: &ParsedInput) -> Result<i32> {
    Ok((&input.elves.iter().chunks(3))
        .into_iter()
        .map(|elves| {
            let intersection = elves
                .map(|bag| {
                    bag.left
                        .chars()
                        .chain(bag.right.chars())
                        .collect::<BTreeSet<char>>()
                })
                .reduce(|bag1, bag2| bag1.intersection(&bag2).copied().collect())
                .unwrap();
            let item = intersection.iter().next().unwrap();

            item_to_priority(*item)
        })
        .sum())
}

#[test]
fn test() {
    let test = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    "#
    .trim();

    assert_task!(parse, task1, test, 157);
    assert_task!(parse, task2, test, 70);
}

aoc_main!(parse, task1, task2);
