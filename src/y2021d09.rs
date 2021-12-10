use aoc::*;
use itertools::Itertools;
use ndarray::{Array1, Array2, ArrayViewMut2};

struct ParsedInput {
    map: Array2<u8>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        bytes::complete::take,
        character::complete::{line_ending, u8},
        combinator::map_res,
        multi::{many1, separated_list1},
        Parser,
    };
    let height = take(1_usize).and_then(u8);
    let row = many1(height);
    let rows = separated_list1(line_ending, row);
    let mut parser = map_res(rows, |rows| {
        let height = rows.len();
        let width = rows.get(0).map(|n| n.len()).unwrap_or_default();
        Array1::from_iter(rows.into_iter().flatten())
            .into_shape((height, width))
            .map(|map| ParsedInput { map })
    });
    parser.parse(input)
}

struct Task1Output(Vec<((usize, usize), u32)>);
impl std::fmt::Debug for Task1Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|(_, n)| n).sum::<u32>())
    }
}

fn task1(input: &ParsedInput) -> Result<Task1Output> {
    Ok({
        Task1Output(
            input
                .map
                .indexed_iter()
                .filter(|&((y, x), &e)| {
                    (y == 0 || e < input.map.get((y - 1, x)).copied().unwrap_or(u8::MAX))
                        && e < input.map.get((y + 1, x)).copied().unwrap_or(u8::MAX)
                        && (x == 0 || e < input.map.get((y, x - 1)).copied().unwrap_or(u8::MAX))
                        && e < input.map.get((y, x + 1)).copied().unwrap_or(u8::MAX)
                })
                .map(|((y, x), &n)| ((y, x), n as u32 + 1))
                .collect(),
        )
    })
}

fn dfssize(map: &mut ArrayViewMut2<'_, bool>, (cy, cx): (usize, usize), sum: &mut usize) {
    let target = if let Some(target) = map.get_mut((cy, cx)) {
        if !*target {
            return;
        }
        target
    } else {
        return;
    };

    *target = false;
    *sum += 1;

    if cy != 0 {
        dfssize(map, (cy - 1, cx), sum);
    }
    dfssize(map, (cy + 1, cx), sum);
    if cx != 0 {
        dfssize(map, (cy, cx - 1), sum);
    }
    dfssize(map, (cy, cx + 1), sum);
}

fn task2(input: &ParsedInput, task1: Task1Output) -> Result<usize> {
    let mut floodmap = input.map.map(|&n| n != 9);
    let mut floodmap = floodmap.view_mut();
    Ok(task1
        .0
        .into_iter()
        .map(|((cy, cx), _)| {
            let mut out = 0;
            dfssize(&mut floodmap, (cy, cx), &mut out);
            out
        })
        .sorted()
        .rev()
        .take(3)
        .product())
}

#[test]
fn test() {
    let input = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
    "#
    .trim();

    assert_task!(parse, task1, input, 15);
    assert_task!(parse, task1 -> task2, input, 1134);
}

aoc_main!(parse, task1 -> task2);
