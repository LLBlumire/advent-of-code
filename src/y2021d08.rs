use std::collections::BTreeSet;

use aoc::*;

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

fn task2(input: &ParsedInput) -> Result<usize> {
    // We know the following just from W.l
    // 2 -> 1
    // 3 -> 7
    // 4 -> 4
    // 7 -> 8
    //
    // W.l (WU7).l (WU4).l
    //  5                  -> 2 3 5
    //  5     3            -> 3
    //  5     2       3    -> 5
    //  5     2       2    -> 2
    //  6                  -> 0 6 9
    //  6             4    -> 9
    //  6     3       3    -> 0
    //  6     2       3    -> 6
    Ok(input
        .segments
        .iter()
        .filter_map(|segment| {
            let (seven, four) = segment.full_cycle.iter().map(|n| (n, n.wires.len())).fold(
                (None, None),
                |(seven, four), (item, n)| match n {
                    3 => (Some(item), four),
                    4 => (seven, Some(item)),
                    _ => (seven, four),
                },
            );
            let seven = &seven?.wires;
            let four = &four?.wires;
            Some(
                segment
                    .output
                    .iter()
                    .rev()
                    .scan(0, |pow, item| {
                        *pow += 1;
                        Some(
                            10_usize.pow(*pow - 1)
                                * match (
                                    item.wires.len(),
                                    seven.intersection(&item.wires).count(),
                                    four.intersection(&item.wires).count(),
                                ) {
                                    (2, _, _) => 1,
                                    (3, _, _) => 7,
                                    (4, _, _) => 4,
                                    (7, _, _) => 8,
                                    (5, 3, _) => 3,
                                    (5, _, 3) => 5,
                                    (5, _, _) => 2,
                                    (6, _, 4) => 9,
                                    (6, 3, _) => 0,
                                    _ => 6,
                                },
                        )
                    })
                    .sum::<usize>(),
            )
        })
        .sum())
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
