use crate::*;
use std::ops::Range;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PartitionV {
    Front,
    Back,
}
impl PartitionV {
    fn is_low(self) -> bool {
        self == PartitionV::Front
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PartitionH {
    Left,
    Right,
}
impl PartitionH {
    fn is_low(self) -> bool {
        self == PartitionH::Left
    }
}
trait RangePartition {
    type Elements;
    fn do_partition(&mut self, low: bool);
    fn finish(self) -> Option<Self::Elements>;
}
impl RangePartition for Range<usize> {
    type Elements = usize;
    fn do_partition(&mut self, low: bool) {
        if low {
            *self = self.start..((self.end + self.start) / 2)
        } else {
            *self = ((self.end + self.start) / 2)..self.end
        }
    }
    fn finish(self) -> Option<usize> {
        if self.len() == 1 {
            Some(self.start)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BoardingPass {
    v_scheme: Vec<PartitionV>,
    h_scheme: Vec<PartitionH>,
}
impl BoardingPass {
    /// Panics if partition scheme is incomplete
    fn get_seat_id(&self) -> usize {
        let mut v_partition = 0..128;
        let mut h_partition = 0..8;
        for &v in self.v_scheme.iter() {
            v_partition.do_partition(v.is_low());
        }
        for &h in self.h_scheme.iter() {
            h_partition.do_partition(h.is_low());
        }
        match (v_partition.finish(), h_partition.finish()) {
            (Some(y), Some(x)) => y * 8 + x,
            _ => panic!("Invalid boarding pass"),
        }
    }
}

#[derive(Debug)]
pub struct ParsedInput {
    passes: Vec<BoardingPass>,
}
impl ParsedInput {
    fn highest_seat_id(&self) -> usize {
        self.passes.iter().map(BoardingPass::get_seat_id).max().unwrap()
    }
    fn lowest_seat_id(&self) -> usize {
        self.passes.iter().map(BoardingPass::get_seat_id).min().unwrap()
    }
    /// Panics if no seat IDs are missing
    fn missing_seat_id_not_front_or_back(&self) -> usize {
        let mut all_seat_ids = self.passes.iter().map(BoardingPass::get_seat_id).collect::<Vec<_>>();
        all_seat_ids.sort_unstable();
        for test_value in self.lowest_seat_id()..self.highest_seat_id() {
            if all_seat_ids.contains(&test_value) {
                continue;
            }
            return test_value;
        }
        panic!()
    }
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let front = map(tag("F"), |_| PartitionV::Front);
    let back = map(tag("B"), |_| PartitionV::Back);
    let left = map(tag("L"), |_| PartitionH::Left);
    let right = map(tag("R"), |_| PartitionH::Right);
    let vertical = alt((front, back));
    let horizontal = alt((left, right));
    let scheme = tuple((many1(vertical), many1(horizontal)));
    let pass = map(scheme, |(v_scheme, h_scheme)| BoardingPass { v_scheme, h_scheme });
    let passes = separated_list1(line_ending, pass);
    let mut parsed = map(passes, |passes| ParsedInput { passes });
    Ok(parsed(input)?)
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output { task1: input.highest_seat_id(), task2: input.missing_seat_id_not_front_or_back() })
}
