use aoc::*;
use itertools::Itertools;

struct ParsedInput {
    elves: Vec<Elf>,
}

struct Elf {
    inventory: Vec<i32>,
}

impl Elf {
    fn total_calories_carried(&self) -> i32 {
        self.inventory.iter().sum()
    }
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{i32 as number, line_ending},
        multi::separated_list1,
        Parser,
    };

    let elf = separated_list1(line_ending, number).map(|inventory| Elf { inventory });
    let mut parser =
        separated_list1(line_ending.and(line_ending), elf).map(|elves| ParsedInput { elves });

    parser.parse(input)
}

fn task1(input: &ParsedInput) -> Result<i32> {
    Ok(input
        .elves
        .iter()
        .map(Elf::total_calories_carried)
        .max()
        .unwrap())
}

fn task2(input: &ParsedInput) -> Result<i32> {
    Ok(input
        .elves
        .iter()
        .map(Elf::total_calories_carried)
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum())
}

#[test]
fn test() {
    let test_input = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
    "
    .trim();

    assert_task!(parse, task1, test_input, 24000);
    assert_task!(parse, task2, test_input, 45000);
}

aoc_main!(parse, task1, task2);
