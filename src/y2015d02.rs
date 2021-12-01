use aoc::*;

struct ParsedInput {
    boxes: Vec<PresentBox>,
}

#[derive(Copy, Clone)]
struct PresentBox {
    dim1: i32,
    dim2: i32,
    dim3: i32,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{char, i32, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::tuple,
    };
    let dimensions = tuple((i32, char('x'), i32, char('x'), i32));
    let present_box = map(dimensions, |(dim1, _, dim2, _, dim3)| PresentBox {
        dim1,
        dim2,
        dim3,
    });
    let boxes = separated_list1(line_ending, present_box);
    let mut parse = map(boxes, |boxes| ParsedInput { boxes });
    parse(input)
}

fn task1(input: &ParsedInput) -> Result<i32> {
    Ok(input
        .boxes
        .iter()
        .map(|PresentBox { dim1, dim2, dim3 }| {
            let mut areas = [dim1 * dim2, dim2 * dim3, dim1 * dim3];
            areas.sort_unstable();
            let surface_area = areas.iter().sum::<i32>() * 2;
            let smallest_area = areas[0];
            surface_area + smallest_area
        })
        .sum())
}

fn task2(input: &ParsedInput) -> Result<i32> {
    Ok(input
        .boxes
        .iter()
        .map(|&PresentBox { dim1, dim2, dim3 }| {
            let mut dims = [dim1, dim2, dim3];
            dims.sort_unstable();
            let smallest_perimeter = 2 * (dims[0] + dims[1]);
            let volume = dims.iter().product::<i32>();
            smallest_perimeter + volume
        })
        .sum())
}

#[test]
fn test() {
    assert_task!(parse, task1, "2x3x4", 58);
    assert_task!(parse, task1, "1x1x10", 43);
    assert_task!(parse, task2, "2x3x4", 34);
    assert_task!(parse, task2, "1x1x10", 14);
}

aoc_main!(parse, task1, task2);
