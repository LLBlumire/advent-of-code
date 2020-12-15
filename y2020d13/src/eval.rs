use crate::*;
use std::str::FromStr;

#[derive(Debug)]
enum Bus {
    Id(i64),
    NoData,
}
impl Bus {
    fn id(&self) -> Option<i64> {
        match self {
            Bus::Id(bus) => Some(*bus),
            Bus::NoData => None,
        }
    }
}

#[derive(Debug)]
struct Schedule {
    buses: Vec<Bus>,
}

#[derive(Debug)]
pub struct ParsedInput {
    start_time: i64,
    schedule: Schedule,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let number = |i| map_res(digit1, FromStr::from_str)(i);
    let bus_id = map(number, Bus::Id);
    let no_data = map(tag("x"), |_| Bus::NoData);
    let bus = alt((bus_id, no_data));
    let schedule = map(separated_list1(char(','), bus), |buses| Schedule { buses });
    let mut parsed = map(separated_pair(number, line_ending, schedule), |(start_time, schedule)| ParsedInput {
        start_time,
        schedule,
    });
    Ok(parsed(input)?)
}

impl Bus {
    fn minutes_to_depart(&self, from: i64) -> Option<i64> {
        self.id().map(|time| (time - from).rem_euclid(time))
    }
}

#[derive(Debug)]
struct NextBus<'a> {
    bus: &'a Bus,
    time_till_bus: i64,
}
impl Schedule {
    fn next_bus(&self, from: i64) -> Option<NextBus<'_>> {
        self.buses
            .iter()
            .filter_map(|bus| bus.minutes_to_depart(from).map(|time_till_bus| NextBus { bus, time_till_bus }))
            .min_by_key(|a| a.time_till_bus)
    }
    fn find_sequence_start_time(&self) -> i64 {
        let mut ts = 0;
        let mut step = 1;
        for (i, period) in self.buses.iter().enumerate().filter_map(|(i, b)| b.id().map(|b| (i, b))) {
            loop {
                if (i as i64 + ts) % period == 0 {
                    step *= period;
                    break;
                }
                ts += step;
            }
        }
        ts
    }
}

pub type Task1 = i64;
pub type Task2 = i64;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let next_bus = input.schedule.next_bus(input.start_time).ok_or(Error::None)?;
    Ok(Output {
        task1: next_bus.bus.id().ok_or(Error::None)? * next_bus.time_till_bus,
        task2: input.schedule.find_sequence_start_time(),
    })
}
