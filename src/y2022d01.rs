use aoc::*;
use itertools::Itertools;

#[derive(Debug)]
struct ParsedInput {
    elves: Vec<Elf>,
}

#[derive(Debug, Clone)]
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
        combinator::map,
        multi::separated_list1,
        sequence::tuple,
    };

    let elf = map(separated_list1(line_ending, number), |inventory| Elf {
        inventory,
    });
    let mut parser = map(
        separated_list1(tuple((line_ending, line_ending)), elf),
        |elves| ParsedInput { elves },
    );

    parser(input)
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
