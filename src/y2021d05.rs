use aoc::*;
use itertools::Itertools;
use ndarray::Array2;
use nom::multi::separated_list1;

struct ParsedInput {
    lines: Vec<Line>,
}

#[derive(Copy, Clone)]
struct Line {
    a: Coord,
    b: Coord,
}

#[derive(Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        bytes::complete::tag,
        character::complete::{char, line_ending, u32},
        sequence::separated_pair,
        Parser,
    };
    let coord = |i| {
        separated_pair(u32, char(','), u32)
            .map(|(x, y)| Coord {
                x: x as usize,
                y: y as usize,
            })
            .parse(i)
    };
    let line = separated_pair(coord, tag(" -> "), coord).map(|(a, b)| {
        let (a, b) = [a, b]
            .into_iter()
            .minmax_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))
            .into_option()
            .unwrap();
        Line { a, b }
    });
    let mut parser = separated_list1(line_ending, line).map(|lines| ParsedInput { lines });
    parser.parse(input)
}

fn find_field_overlaps(lines: &[Line]) -> usize {
    let (bound_x, bound_y) = lines
        .iter()
        .map(|line| (line.a.x.max(line.b.x), line.a.y.max(line.b.y)))
        .fold((0, 0), |(mx, my), (x, y)| (mx.max(x), my.max(y)));
    let mut field = Array2::from_elem((bound_x as usize + 1, bound_y as usize + 1), 0);
    for line in lines {
        let mut x_pos: Box<dyn Iterator<Item = usize>> = {
            if line.a.x <= line.b.x {
                Box::new(line.a.x..=line.b.x)
            } else {
                Box::new((line.b.x..=line.a.x).rev())
            }
        };
        let mut y_pos: Box<dyn Iterator<Item = usize>> = {
            if line.a.y <= line.b.y {
                Box::new(line.a.y..=line.b.y)
            } else {
                Box::new((line.b.y..=line.a.y).rev())
            }
        };
        let mut last_x = None;
        let mut last_y = None;

        loop {
            let next_x = x_pos.next();
            let next_y = y_pos.next();
            if next_x.is_none() && next_y.is_none() {
                break;
            }
            last_x = next_x.or(last_x);
            last_y = next_y.or(last_y);
            if let (Some(x), Some(y)) = (last_x, last_y) {
                if let Some(v) = field.get_mut((x, y)) {
                    *v += 1
                }
            }
        }
    }
    field.iter().filter(|n| **n >= 2).count()
}

fn task1(input: &ParsedInput) -> Result<usize> {
    let only_horizonal_or_vertical = input
        .lines
        .iter()
        .copied()
        .filter(|line| line.a.x == line.b.x || line.a.y == line.b.y)
        .collect::<Vec<_>>();
    Ok(find_field_overlaps(&only_horizonal_or_vertical))
}

fn task2(input: &ParsedInput) -> Result<usize> {
    Ok(find_field_overlaps(&input.lines))
}

#[test]
fn test() {
    let test = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
    "#
    .trim();
    // assert_task!(parse, task1, test, 5);
    assert_task!(parse, task2, test, 12);
}

aoc_main!(parse, task1, task2);
