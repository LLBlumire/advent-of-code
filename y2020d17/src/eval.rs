use crate::*;
use std::{collections::BTreeSet,
          ops::RangeInclusive};

#[derive(Debug)]
pub struct ParsedInput {
    grid: arr::Array2<bool>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let active = map(char('#'), |_| true);
    let inactive = map(char('.'), |_| false);
    let row = many1(alt((active, inactive)));
    let grid = separated_list1(line_ending, row);
    let parsed_grid = map_res(grid, |g: Vec<Vec<bool>>| {
        let shape = (g.len(), g.get(0).as_ref().unwrap().len());
        g.into_iter().flatten().collect::<arr::Array1<bool>>().into_shape(shape)
    });
    let mut parsed = map(parsed_grid, |grid| ParsedInput { grid });
    Ok(parsed(input)?)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
struct Cd3 {
    x: isize,
    y: isize,
    z: isize,
}
impl Cd3 {
    fn neighbours(&self) -> impl Iterator<Item = Cd3> {
        let Cd3 { x, y, z } = *self;
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .filter(|((dx, dy), dz)| *dx != 0 || *dy != 0 || *dz != 0)
            .map(move |((dx, dy), dz)| Cd3 { x: x + dx, y: y + dy, z: z + dz })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
struct Cd4 {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}
impl Cd4 {
    fn neighbours(&self) -> impl Iterator<Item = Cd4> {
        let Cd4 { w, x, y, z } = *self;
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(((dw, dx), dy), dz)| *dw != 0 || *dx != 0 || *dy != 0 || *dz != 0)
            .map(move |(((dw, dx), dy), dz)| Cd4 { w: w + dw, x: x + dx, y: y + dy, z: z + dz })
    }
}

#[derive(Debug, Clone)]
pub struct ConwayCubeSimulator {
    cubes: BTreeSet<Cd3>,
    range_x: RangeInclusive<isize>,
    range_y: RangeInclusive<isize>,
    range_z: RangeInclusive<isize>,
}
impl ConwayCubeSimulator {
    fn has(&mut self, cd: Cd3) -> bool {
        self.cubes.contains(&cd)
    }
    fn set(&mut self, cd: Cd3) {
        self.cubes.insert(cd);
        self.range_x = (*self.range_x.start()).min(cd.x - 1)..=(*self.range_x.end()).max(cd.x + 1);
        self.range_y = (*self.range_y.start()).min(cd.y - 1)..=(*self.range_y.end()).max(cd.y + 1);
        self.range_z = (*self.range_z.start()).min(cd.z - 1)..=(*self.range_z.end()).max(cd.z + 1);
    }
    fn unset(&mut self, cd: Cd3) {
        self.cubes.remove(&cd);
    }
    fn from_active_positions<I: IntoIterator<Item = Cd3>>(i: I) -> ConwayCubeSimulator {
        let mut sim = ConwayCubeSimulator { cubes: BTreeSet::new(), range_x: -1..=1, range_y: -1..=1, range_z: -1..=1 };
        for cube in i {
            sim.set(cube);
        }
        sim
    }
    fn step(&mut self) {
        let mut next = self.clone();
        for place in self
            .range_x
            .clone()
            .cartesian_product(self.range_y.clone())
            .cartesian_product(self.range_z.clone())
            .map(|((x, y), z)| Cd3 { x, y, z })
        {
            let neighbours: isize = place.neighbours().map(|neighbour| if self.has(neighbour) { 1 } else { 0 }).sum();
            let this = self.has(place);
            match (this, neighbours) {
                (true, 2) | (true, 3) => {}
                (true, _) => next.unset(place),
                (false, 3) => next.set(place),
                (false, _) => {}
            }
        }
        *self = next;
    }
    fn step_for(&mut self, times: usize) {
        for _ in 0..times {
            self.step()
        }
    }
    fn count_active(&self) -> usize {
        self.cubes.len()
    }
}

#[derive(Debug, Clone)]
pub struct ConwayCubeSimulator4d {
    cubes: BTreeSet<Cd4>,
    range_w: RangeInclusive<isize>,
    range_x: RangeInclusive<isize>,
    range_y: RangeInclusive<isize>,
    range_z: RangeInclusive<isize>,
}
impl ConwayCubeSimulator4d {
    fn has(&mut self, cd: Cd4) -> bool {
        self.cubes.contains(&cd)
    }
    fn set(&mut self, cd: Cd4) {
        self.cubes.insert(cd);
        self.range_w = (*self.range_w.start()).min(cd.w - 1)..=(*self.range_w.end()).max(cd.w + 1);
        self.range_x = (*self.range_x.start()).min(cd.x - 1)..=(*self.range_x.end()).max(cd.x + 1);
        self.range_y = (*self.range_y.start()).min(cd.y - 1)..=(*self.range_y.end()).max(cd.y + 1);
        self.range_z = (*self.range_z.start()).min(cd.z - 1)..=(*self.range_z.end()).max(cd.z + 1);
    }
    fn unset(&mut self, cd: Cd4) {
        self.cubes.remove(&cd);
    }
    fn from_active_positions<I: IntoIterator<Item = Cd4>>(i: I) -> ConwayCubeSimulator4d {
        let mut sim = ConwayCubeSimulator4d {
            cubes: BTreeSet::new(),
            range_w: -1..=1,
            range_x: -1..=1,
            range_y: -1..=1,
            range_z: -1..=1,
        };
        for cube in i {
            sim.set(cube);
        }
        sim
    }
    fn step(&mut self) {
        let mut next = self.clone();
        for place in self
            .range_w
            .clone()
            .cartesian_product(self.range_x.clone())
            .cartesian_product(self.range_y.clone())
            .cartesian_product(self.range_z.clone())
            .map(|(((w, x), y), z)| Cd4 { w, x, y, z })
        {
            let neighbours: isize = place.neighbours().map(|neighbour| if self.has(neighbour) { 1 } else { 0 }).sum();
            let this = self.has(place);
            match (this, neighbours) {
                (true, 2) | (true, 3) => {}
                (true, _) => next.unset(place),
                (false, 3) => next.set(place),
                (false, _) => {}
            }
        }
        *self = next;
    }
    fn step_for(&mut self, times: usize) {
        for _ in 0..times {
            self.step()
        }
    }
    fn count_active(&self) -> usize {
        self.cubes.len()
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let mut sim = ConwayCubeSimulator::from_active_positions(
        input
            .grid
            .indexed_iter()
            .filter_map(|((y, x), v)| Some(Cd3 { x: x as isize, y: y as isize, z: 0 }).filter(|_| *v)),
    );
    sim.step_for(6);
    let mut sim4d = ConwayCubeSimulator4d::from_active_positions(
        input
            .grid
            .indexed_iter()
            .filter_map(|((y, x), v)| Some(Cd4 { w: 0, x: x as isize, y: y as isize, z: 0 }).filter(|_| *v)),
    );
    sim4d.step_for(6);
    Ok(Output { task1: sim.count_active(), task2: sim4d.count_active() })
}
