use crate::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct XmasCode {
    numbers: Vec<i32>,
}

#[derive(Debug)]
pub struct ParsedInput {
    code: XmasCode,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let number = map_res(digit1, FromStr::from_str);
    let mut parsed = map(separated_list1(line_ending, number), |numbers| ParsedInput { code: XmasCode { numbers } });
    Ok(parsed(input)?)
}

impl XmasCode {
    fn window(&self, size: usize, offset: usize) -> (&[i32], i32) {
        (&self.numbers[offset..offset + size], self.numbers[offset + size])
    }
    fn find_weak_key(&self, size: usize) -> i32 {
        for offset in 0.. {
            let (window, seek) = self.window(size, offset);
            if window.iter().tuple_combinations().map(|(a, b)| a + b).any(|n| n == seek) {
                continue;
            }
            return seek;
        }
        panic!()
    }
    fn seek_weakness(&self, weak_key: i32) -> i32 {
        for window_size in 2.. {
            for (seek, low, high) in self
                .numbers
                .windows(window_size)
                .map(|n| n.iter().fold((0, i32::MAX, 0), |(sum, min, max), n| (sum + n, min.min(*n), max.max(*n))))
            {
                if seek == weak_key {
                    return low + high;
                }
            }
        }
        panic!()
    }
}

pub type Task1 = i32;
pub type Task2 = i32;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let task1 = input.code.find_weak_key(25);
    Ok(Output { task1, task2: input.code.seek_weakness(task1) })
}
