use std::collections::BTreeSet;

use aoc::*;
use itertools::Itertools;

#[derive(Debug)]
struct ParsedInput {
    segments: Vec<SegmentDisplayInfo>,
}

#[derive(Debug)]
struct SegmentDisplayInfo {
    full_cycle: Vec<SegmentInstruction>,
    output: Vec<SegmentInstruction>,
}

#[derive(Debug)]
struct SegmentInstruction {
    wires: BTreeSet<SegmentWire>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(usize)]
enum SegmentWire {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        bytes::complete::tag,
        character::complete::{char, line_ending},
        combinator::{map, value},
        multi::{many1, separated_list1},
        Parser,
    };
    let segment_wire = |i| {
        value(SegmentWire::A, char('a'))
            .or(value(SegmentWire::B, char('b')))
            .or(value(SegmentWire::C, char('c')))
            .or(value(SegmentWire::D, char('d')))
            .or(value(SegmentWire::E, char('e')))
            .or(value(SegmentWire::F, char('f')))
            .or(value(SegmentWire::G, char('g')))
            .parse(i)
    };
    let segment_instruction = |i| {
        map(many1(segment_wire), |i| SegmentInstruction {
            wires: i.into_iter().collect(),
        })
        .parse(i)
    };
    let segment_instructions = |i| separated_list1(char(' '), segment_instruction).parse(i);
    let segment_display_info = map(
        segment_instructions
            .and(tag(" | "))
            .and(segment_instructions),
        |((full_cycle, _), output)| SegmentDisplayInfo { full_cycle, output },
    );
    let mut parser = map(
        separated_list1(line_ending, segment_display_info),
        |segments| ParsedInput { segments },
    );
    parser.parse(input)
}

fn task1(input: &ParsedInput) -> Result<usize> {
    Ok(input
        .segments
        .iter()
        .flat_map(|segment| segment.output.iter())
        .map(|wire| wire.wires.len())
        .filter(|&n| n == 2 || n == 4 || n == 3 || n == 7)
        .count())
}

fn task2(input: &ParsedInput) -> Result<u32> {
    let all_permutations = segments().into_iter().permutations(7).collect::<Vec<_>>();
    Ok(input
        .segments
        .iter()
        .filter_map(|segment| {
            let permutation = all_permutations
                .iter()
                .find(|permutation| is_legal_permutation(permutation, &segment.full_cycle))?;

            Some(
                segment
                    .output
                    .iter()
                    .rev()
                    .scan(0, |p, segment| {
                        *p += 1;
                        Some(10_u32.pow(*p - 1) * decode(permutation, segment) as u32)
                    })
                    .sum::<u32>(),
            )
        })
        .sum())
}

fn segments() -> Vec<SegmentWire> {
    use SegmentWire::*;
    vec![A, B, C, D, E, F, G]
}

fn nummap() -> Vec<BTreeSet<SegmentWire>> {
    use SegmentWire::*;
    vec![
        BTreeSet::from_iter(vec![A, B, C, E, F, G]),
        BTreeSet::from_iter(vec![C, F]),
        BTreeSet::from_iter(vec![A, C, D, E, G]),
        BTreeSet::from_iter(vec![A, C, D, F, G]),
        BTreeSet::from_iter(vec![B, C, D, F]),
        BTreeSet::from_iter(vec![A, B, D, F, G]),
        BTreeSet::from_iter(vec![A, B, D, E, F, G]),
        BTreeSet::from_iter(vec![A, C, F]),
        BTreeSet::from_iter(vec![A, B, C, D, E, F, G]),
        BTreeSet::from_iter(vec![A, B, C, D, F, G]),
    ]
}

fn is_legal_permutation(permutation: &[SegmentWire], test: &[SegmentInstruction]) -> bool {
    let nums = BTreeSet::from_iter(nummap());
    let permuted_test = test
        .iter()
        .map(|wire| permute_map(permutation, &wire.wires))
        .collect::<BTreeSet<_>>();
    nums == permuted_test
}

fn permute_map(
    permutation: &[SegmentWire],
    input: &BTreeSet<SegmentWire>,
) -> BTreeSet<SegmentWire> {
    let segments = segments();
    input
        .iter()
        .copied()
        .filter_map(|n| permutation.iter().find_position(|i| i == &&n))
        .filter_map(|(n, _)| segments.get(n))
        .copied()
        .collect()
}
fn decode(permutation: &[SegmentWire], value: &SegmentInstruction) -> usize {
    let decode_map = permute_map(permutation, &value.wires);
    nummap()
        .iter()
        .find_position(|n| n == &&decode_map)
        .unwrap()
        .0
}

#[test]
fn test() {
    let test = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "#
    .trim();
    assert_task!(parse, task1, test, 26);
    assert_task!(parse, task2, test, 61229);
}

aoc_main!(parse, task1, task2);
