use aoc::*;
use ndarray::{Array1, Array2, ArrayView2};

struct ParsedInput {
    world: Array2<bool>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        branch::alt,
        character::complete::{char, line_ending},
        combinator::{map, map_res, value},
        multi::{many1, separated_list1},
    };
    let empty = value(false, char('.'));
    let tree = value(true, char('#'));
    let location = alt((empty, tree));
    let row = many1(location);
    let rowcol = separated_list1(line_ending, row);
    let world = map_res(rowcol, |v| {
        let shape = (v.len(), v[0].len());
        v.into_iter()
            .flatten()
            .collect::<Array1<_>>()
            .into_shape(shape)
    });
    let mut parsed = map(world, |world| ParsedInput { world });
    parsed(input)
}

fn index(map: ArrayView2<bool>, x: usize, y: usize) -> Option<bool> {
    let (height, width) = {
        let shape = map.shape();
        (shape[0], shape[1])
    };
    if y >= height {
        None
    } else {
        map.get((y, x % width)).copied()
    }
}

fn count_true_slope(map: ArrayView2<bool>, dx: usize, dy: usize) -> usize {
    (0..)
        .step_by(dx)
        .zip((0..).step_by(dy))
        .map(|(x, y)| index(map, x, y))
        .take_while(Option::is_some)
        .filter(|n| n.unwrap_or_default())
        .count()
}

fn task1(input: &ParsedInput) -> Result<usize> {
    Ok(count_true_slope(input.world.view(), 3, 1))
}

fn task2(input: &ParsedInput) -> Result<usize> {
    let view = input.world.view();
    Ok([(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(dx, dy)| count_true_slope(view, dx, dy))
        .product())
}

#[test]
fn test() {
    let input = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
    "
    .trim();
    assert_task!(parse, task1, input, 7);
    assert_task!(parse, task2, input, 336);
}

aoc_main!(parse, task1, task2);
