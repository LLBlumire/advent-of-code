use aoc::*;
use itertools::Itertools;
use std::{fmt::Debug, ops::Range};

struct ParsedInput {
    passes: Vec<BoardingPass>,
}

pub struct BoardingPass {
    v_scheme: Vec<PartitionV>,
    h_scheme: Vec<PartitionH>,
}

#[derive(Copy, Clone)]
enum PartitionV {
    Front,
    Back,
}

#[derive(Copy, Clone)]
enum PartitionH {
    Left,
    Right,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        branch::alt,
        character::complete::{char, line_ending},
        combinator::{map, value},
        multi::{many1, separated_list1},
        sequence::tuple,
    };
    let front = value(PartitionV::Front, char('F'));
    let back = value(PartitionV::Back, char('B'));
    let left = value(PartitionH::Left, char('L'));
    let right = value(PartitionH::Right, char('R'));
    let vertical = alt((front, back));
    let horizontal = alt((left, right));
    let scheme = tuple((many1(vertical), many1(horizontal)));
    let pass = map(scheme, |(v_scheme, h_scheme)| BoardingPass {
        v_scheme,
        h_scheme,
    });
    let passes = separated_list1(line_ending, pass);
    let mut parse = map(passes, |passes| ParsedInput { passes });
    parse(input)
}

trait Partition {
    fn is_low(&self) -> bool;
    fn partition(&self, range: Range<usize>) -> Range<usize> {
        if self.is_low() {
            range.start..((range.end + range.start) / 2)
        } else {
            ((range.end + range.start) / 2)..range.end
        }
    }
}

impl Partition for PartitionV {
    fn is_low(&self) -> bool {
        matches!(self, &PartitionV::Front)
    }
}

impl Partition for PartitionH {
    fn is_low(&self) -> bool {
        matches!(self, &PartitionH::Left)
    }
}

impl BoardingPass {
    fn get_id(&self) -> usize {
        let mut v_partition = 0..128;
        let mut h_partition = 0..8;
        for partition_v in self.v_scheme.iter() {
            v_partition = partition_v.partition(v_partition);
        }
        for partition_h in self.h_scheme.iter() {
            h_partition = partition_h.partition(h_partition);
        }
        v_partition.start * 8 + h_partition.start
    }
}

struct Task1Result {
    inner: Vec<usize>,
}
impl Debug for Task1Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner.last().copied().unwrap_or_default())
    }
}

fn task1(input: &ParsedInput) -> Result<Task1Result> {
    Ok(Task1Result {
        inner: input
            .passes
            .iter()
            .map(BoardingPass::get_id)
            .sorted_unstable()
            .collect(),
    })
}

fn task2(_input: &ParsedInput, mut all_seat_ids: Task1Result) -> Result<usize> {
    let mut prev_seat = all_seat_ids.inner.last().ok_or("expected seats")? + 1;
    while let Some(seat) = all_seat_ids.inner.pop() {
        if seat + 1 == prev_seat {
            prev_seat = seat;
        } else {
            return Ok(seat + 1);
        }
    }
    Err("not found".into())
}

#[test]
fn test() {
    assert_task!(parse, task1, "BFFFBBFRRR", 567);
    assert_task!(parse, task1, "FFFBBBFRRR", 119);
    assert_task!(parse, task1, "BBFFBBFRLL", 820);
}

aoc_main!(parse, task1 -> task2);
