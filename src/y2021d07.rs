use aoc::*;

struct ParsedInput {
    crab_places: Vec<i64>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{char, i64},
        combinator::map,
        multi::separated_list1,
        Parser,
    };
    let mut parser = map(separated_list1(char(','), i64), |crab_places| ParsedInput {
        crab_places,
    });
    parser.parse(input)
}

fn task1(input: &ParsedInput) -> Result<i64> {
    let mut positions = input.crab_places.clone();
    positions.sort_unstable();
    let target = *positions.get(positions.len() / 2).ok_or("No data")?;
    Ok(positions.into_iter().map(|n| (target - n).abs()).sum())
}

fn task2(input: &ParsedInput) -> Result<i64> {
    let positions = input.crab_places.clone();
    let max = *input.crab_places.iter().max().ok_or("no max found")?;
    Ok((0..max)
        .into_iter()
        .map(|target| {
            positions
                .iter()
                .map(|n| {
                    let n = (target - n).abs();
                    (n * (n + 1)) / 2
                })
                .sum()
        })
        .min()
        .ok_or("Cannot find data")?)
}

#[test]
fn test() {
    assert_task!(parse, task1, "16,1,2,0,4,2,7,1,2,14", 37);
    assert_task!(parse, task2, "16,1,2,0,4,2,7,1,2,14", 206);
}

aoc_main!(parse, task1, task2);
