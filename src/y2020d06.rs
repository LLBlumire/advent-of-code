use aoc::*;
use std::collections::BTreeSet;

struct ParsedInput {
    groups: Vec<Group>,
}
struct Group {
    members: Vec<Person>,
}
struct Person {
    answers: BTreeSet<char>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{alpha1, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::tuple,
    };
    let person = map(alpha1, |answers: &str| Person {
        answers: answers.chars().collect(),
    });
    let group = map(separated_list1(line_ending, person), |members| Group {
        members,
    });
    let mut parse = map(
        separated_list1(tuple((line_ending, line_ending)), group),
        |groups| ParsedInput { groups },
    );
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<usize> {
    Ok(input
        .groups
        .iter()
        .map(|group| {
            group
                .members
                .iter()
                .flat_map(|person| person.answers.iter())
                .copied()
                .collect::<BTreeSet<char>>()
                .len()
        })
        .sum())
}

fn task2(input: &ParsedInput) -> Result<usize> {
    Ok(input
        .groups
        .iter()
        .map(|group| {
            group
                .members
                .iter()
                .map(|person| &person.answers)
                .fold(('a'..='z').collect(), |a, a2| {
                    a2.intersection(&a).cloned().collect()
                })
                .len()
        })
        .sum())
}

#[test]
fn test() {
    let input = "
abc

a
b
c

ab
ac

a
a
a
a

b
    "
    .trim();

    assert_task!(parse, task1, input, 11);
    assert_task!(parse, task2, input, 6);
}

aoc_main!(parse, task1, task2);
