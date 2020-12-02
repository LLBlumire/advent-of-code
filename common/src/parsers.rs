use nom::{character::complete::*, combinator::*, multi::*, *};

fn number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}
pub fn number_list(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(line_ending, number)(input)
}