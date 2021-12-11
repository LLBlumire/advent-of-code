use aoc::*;
use ndarray::{Array1, Array2, ArrayViewMut2};

struct ParsedInput {
    grid: Array2<u8>,
}

const GRID_SHAPE: usize = 10;

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        bytes::complete::take,
        character::complete::{line_ending, u8},
        combinator::{map, map_res},
        multi::{many1, separated_list1},
        Parser,
    };

    let octopus_value = take(1_usize).and_then(u8);
    let row = many1(octopus_value);
    let rows = separated_list1(line_ending, row);
    let grid = map_res(rows, |rows| {
        rows.into_iter()
            .flatten()
            .collect::<Array1<_>>()
            .into_shape((GRID_SHAPE, GRID_SHAPE))
    });
    let mut parser = map(grid, |grid| ParsedInput { grid });
    parser.parse(input)
}

fn increment(view: &mut ArrayViewMut2<'_, u8>, target: (usize, usize)) {
    let target_octopus = if let Some(v) = view.get_mut(target) {
        v
    } else {
        return;
    };
    *target_octopus += 1;
}

fn flash(
    view: &mut ArrayViewMut2<'_, u8>,
    target @ (cy, cx): (usize, usize),
    flashes: &mut ArrayViewMut2<'_, bool>,
) {
    let target_octopus = if let Some(v) = view.get_mut(target) {
        v
    } else {
        return;
    };
    let target_flashed = if let Some(v) = flashes.get_mut(target) {
        v
    } else {
        return;
    };
    if *target_octopus > 9 && !*target_flashed {
        *target_flashed = true;
        for y in cy.saturating_sub(1)..=(cy + 1) {
            for x in cx.saturating_sub(1)..=(cx + 1) {
                if y == cy && x == cx {
                    continue;
                }
                increment(view, (y, x));
                flash(view, (y, x), flashes);
            }
        }
    }
}

fn reset_flash(
    view: &mut ArrayViewMut2<'_, u8>,
    target: (usize, usize),
    flash: &mut ArrayViewMut2<'_, bool>,
    counter: &mut usize,
) {
    let target_octopus = if let Some(v) = view.get_mut(target) {
        v
    } else {
        return;
    };
    let target_flashed = if let Some(v) = flash.get_mut(target) {
        v
    } else {
        return;
    };
    if *target_flashed {
        *target_octopus = 0;
        *counter += 1;
    }
    *target_flashed = false;
}

fn task1(input: &ParsedInput) -> Result<usize> {
    let mut grid = input.grid.clone();
    let mut grid = grid.view_mut();
    let mut flashes = Array2::from_elem((GRID_SHAPE, GRID_SHAPE), false);
    let mut flashes = flashes.view_mut();
    let mut count = 0;
    for _ in 0..100 {
        for y in 0..GRID_SHAPE {
            for x in 0..GRID_SHAPE {
                increment(&mut grid, (y, x));
            }
        }
        for y in 0..GRID_SHAPE {
            for x in 0..GRID_SHAPE {
                flash(&mut grid, (y, x), &mut flashes);
            }
        }
        for y in 0..GRID_SHAPE {
            for x in 0..GRID_SHAPE {
                reset_flash(&mut grid, (y, x), &mut flashes, &mut count);
            }
        }
    }
    Ok(count)
}

fn task2(input: &ParsedInput) -> Result<usize> {
    let mut grid = input.grid.clone();
    let mut grid = grid.view_mut();
    let mut flashes = Array2::from_elem((GRID_SHAPE, GRID_SHAPE), false);
    let mut flashes = flashes.view_mut();
    let mut count = 0;
    for i in 1.. {
        for y in 0..GRID_SHAPE {
            for x in 0..GRID_SHAPE {
                increment(&mut grid, (y, x));
            }
        }
        for y in 0..GRID_SHAPE {
            for x in 0..GRID_SHAPE {
                flash(&mut grid, (y, x), &mut flashes);
            }
        }
        if flashes.iter().all(|f| *f) {
            return Ok(i);
        }
        for y in 0..GRID_SHAPE {
            for x in 0..GRID_SHAPE {
                reset_flash(&mut grid, (y, x), &mut flashes, &mut count);
            }
        }
    }
    unreachable!()
}

#[test]
fn test() {
    let input = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
    "#
    .trim();

    assert_task!(parse, task1, input, 1656);
    assert_task!(parse, task2, input, 195);
}

aoc_main!(parse, task1, task2);
