use crate::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParsedInput {
    boxes: Vec<PresentBox>,
}

#[derive(Copy, Clone, Debug)]
struct PresentBox {
    dim1: i32,
    dim2: i32,
    dim3: i32,
}
impl PresentBox {
    fn wrapping_area(&self) -> i32 {
        let PresentBox { dim1, dim2, dim3 } = *self;
        let mut areas = [dim1 * dim2, dim2 * dim3, dim1 * dim3];
        areas.sort_unstable();
        let surface_area = areas.iter().sum::<i32>() * 2;
        let smallest_area = areas[0];
        surface_area + smallest_area
    }
    fn ribon_length(&self) -> i32 {
        let PresentBox { dim1, dim2, dim3 } = *self;
        let mut dims = [dim1, dim2, dim3];
        dims.sort_unstable();
        let smallest_perimeter = 2 * (dims[0] + dims[1]);
        let volume = dims.iter().product::<i32>();
        smallest_perimeter + volume
    }
}

pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let number = |i| map_res(digit1, FromStr::from_str)(i);
    let dimensions = tuple((number, char('x'), number, char('x'), number));
    let present_box = map(dimensions, |(dim1, _, dim2, _, dim3)| PresentBox { dim1, dim2, dim3 });
    let boxes = separated_list0(line_ending, present_box);
    let mut parsed = map(boxes, |boxes| ParsedInput { boxes });
    Ok(parsed(input)?)
}

pub type Task1 = i32;
pub type Task2 = i32;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input.boxes.iter().map(|n| n.wrapping_area()).sum(),
        task2: input.boxes.iter().map(|n| n.ribon_length()).sum(),
    })
}
