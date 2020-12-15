use crate::*;
use common::arr;
use std::ops::Range;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Floor,
    EmptySeat,
    FullSeat,
}

#[derive(Debug)]
pub struct ParsedInput {
    grid: arr::Array2<State>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let seat = |i| map(tag("L"), |_| State::EmptySeat)(i);
    let floor = |i| map(tag("."), |_| State::Floor)(i);
    let state = |i| alt((seat, floor))(i);
    let row = |i| many1(state)(i);
    let grid = separated_list1(line_ending, row);
    let parsed_grid = map_res(grid, |g: Vec<Vec<State>>| {
        let shape = (g.len(), g.get(0).as_ref().unwrap().len());
        g.into_iter().flatten().collect::<arr::Array1<State>>().into_shape(shape)
    });
    let mut parsed = map(parsed_grid, |grid| ParsedInput { grid });
    Ok(parsed(input)?)
}

struct SeatSimulator {
    state: arr::Array2<State>,
}
impl SeatSimulator {
    fn update_state(state: &mut State, surrounding: arr::ArrayView2<State>) {
        match *state {
            State::Floor => {}
            State::EmptySeat =>
                if surrounding.indexed_iter().filter(|(_, n)| **n == State::FullSeat).count() == 0 {
                    *state = State::FullSeat;
                },
            State::FullSeat =>
                if surrounding.indexed_iter().filter(|(_, n)| **n == State::FullSeat).count() >= 5 {
                    *state = State::EmptySeat;
                },
        }
    }
    fn get_adjacency_shape(xmax: usize, ymax: usize, x: usize, y: usize) -> (Range<usize>, Range<usize>) {
        (x.saturating_sub(1)..(x + 2).min(xmax), y.saturating_sub(1)..(y + 2).min(ymax))
    }
    fn step(&mut self, last: &arr::Array2<State>) {
        let (xmax, ymax) = {
            let shape = self.state.shape();
            (shape[0], shape[1])
        };
        for ((x, y), n) in self.state.indexed_iter_mut() {
            let (xs, ys) = SeatSimulator::get_adjacency_shape(xmax, ymax, x, y);
            let view = last.slice(arr::s![xs, ys]);
            SeatSimulator::update_state(n, view);
        }
    }
    fn simulate(initial_state: arr::Array2<State>) -> SeatSimulator {
        let mut state = SeatSimulator { state: initial_state };
        loop {
            let last = state.state.clone();
            state.step(&last);
            if state.state == last {
                break;
            }
        }
        state
    }
}

struct SeatSimulatorTwo {
    state: arr::Array2<State>,
}
impl SeatSimulatorTwo {
    fn update_state(state: &mut State, view: arr::ArrayView2<State>, (x, y): (usize, usize)) {
        let directions: &[(isize, isize)] = &[(0, 1), (1, 0), (1, 1), (0, -1), (-1, 0), (1, -1), (-1, 1), (-1, -1)];
        let x = x as isize;
        let y = y as isize;
        let num_full = directions
            .iter()
            .map(|(dx, dy)| {
                for i in 1.. {
                    let xp = (x + (dx * i)) as usize;
                    let yp = (y + (dy * i)) as usize;
                    if let Some(state) = view.get((xp, yp)) {
                        match *state {
                            State::Floor => continue,
                            State::EmptySeat => return State::EmptySeat,
                            State::FullSeat => return State::FullSeat,
                        }
                    } else {
                        return State::Floor;
                    }
                }
                unreachable!()
            })
            .filter(|n| *n == State::FullSeat)
            .count();

        match *state {
            State::EmptySeat if num_full == 0 => *state = State::FullSeat,
            State::FullSeat if num_full >= 5 => *state = State::EmptySeat,
            _ => {}
        }
    }
    fn step(&mut self, last: &arr::Array2<State>) {
        let data_source: Vec<((usize, usize), &mut State)> = self.state.indexed_iter_mut().collect();
        data_source.into_iter().for_each(|((x, y), n)| {
            SeatSimulatorTwo::update_state(n, last.view(), (x, y));
        });
    }
    fn simulate(initial_state: arr::Array2<State>) -> SeatSimulatorTwo {
        let mut state = SeatSimulatorTwo { state: initial_state };
        loop {
            let last = state.state.clone();
            state.step(&last);
            if state.state == last {
                break;
            }
        }
        state
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let sim = SeatSimulator::simulate(input.grid.clone());
    let sim2 = SeatSimulatorTwo::simulate(input.grid);
    Ok(Output {
        task1: sim.state.iter().filter(|n| **n == State::FullSeat).count(),
        task2: sim2.state.iter().filter(|n| **n == State::FullSeat).count(),
    })
}
