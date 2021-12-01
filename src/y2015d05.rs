use aoc::*;
use itertools::Itertools;

struct ParsedInput<'a> {
    strings: Vec<&'a str>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{alpha1, line_ending},
        combinator::map,
        multi::separated_list1,
    };
    let strings = separated_list1(line_ending, alpha1);
    let mut parse = map(strings, |strings| ParsedInput { strings });
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<usize> {
    fn nice(input: &impl AsRef<str>) -> bool {
        let input = input.as_ref();
        let vowel_count = input.chars().filter(|c| "aeiou".contains(*c)).count();
        let double_letter = input.chars().tuple_windows().any(|(a, b)| a == b);
        let has_ab = input.contains("ab");
        let has_cd = input.contains("cd");
        let has_pq = input.contains("pq");
        let has_xy = input.contains("xy");
        vowel_count >= 3 && double_letter && !has_ab && !has_cd && !has_pq && !has_xy
    }
    Ok(input.strings.iter().filter(nice).count())
}

fn task2(input: &ParsedInput) -> Result<usize> {
    fn nice(input: &impl AsRef<str>) -> bool {
        let input = input.as_ref();
        let pair_repeats = input
            .chars()
            .tuple_windows()
            .enumerate()
            .any(|(i, (a, b))| {
                input
                    .chars()
                    .skip(i + 2)
                    .tuple_windows()
                    .any(|(c, d)| a == c && b == d)
            });
        let skip_repeats = input.chars().tuple_windows().any(|(a, _, c)| a == c);
        pair_repeats && skip_repeats
    }
    Ok(input.strings.iter().filter(nice).count())
}

#[test]
fn test() {
    assert_task!(parse, task1, "ugknbfddgicrmopn", 1);
    assert_task!(parse, task1, "aaa", 1);
    assert_task!(parse, task1, "jchzalrnumimnmhp", 0);
    assert_task!(parse, task1, "haegwjzuvuyypxyu", 0);
    assert_task!(parse, task1, "dvszwmarrgswjxmb", 0);
    assert_task!(parse, task2, "qjhvhtzxzqqjkmpb", 1);
    assert_task!(parse, task2, "xxyxx", 1);
    assert_task!(parse, task2, "uurcxstgmygtbstg", 0);
    assert_task!(parse, task2, "ieodomkazucvgmuy", 0);
}

aoc_main!(parse, task1, task2);
