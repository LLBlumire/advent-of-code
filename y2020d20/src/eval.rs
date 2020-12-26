use crate::*;
use std::{collections::BTreeMap,
          str::FromStr};

type TileId = usize;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Tile {
    id: TileId,
    data: arr::Array2<bool>,
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let shape = self.data.shape();
        let mut buf = String::new();
        for y in 0..shape[0] {
            buf += "\n";
            for x in 0..shape[1] {
                match self.data.get((y, x)).unwrap() {
                    true => buf += "#",
                    false => buf += ".",
                }
            }
        }
        write!(f, "{}", buf)
    }
}

#[derive(Debug)]
pub struct ParsedInput {
    tiles: Vec<Tile>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let number = |i| map_res(digit1, FromStr::from_str)(i);
    let tile_id = delimited(tag("Tile "), number, tuple((tag(":"), line_ending)));
    let active = map(char('#'), |_| true);
    let inactive = map(char('.'), |_| false);
    let row = many1(alt((active, inactive)));
    let grid = separated_list1(line_ending, row);
    let parsed_grid = map_res(grid, |g: Vec<Vec<bool>>| {
        let shape = (g.len(), g.get(0).as_ref().unwrap().len());
        g.into_iter().flatten().collect::<arr::Array1<bool>>().into_shape(shape)
    });
    let tile = map(tuple((tile_id, parsed_grid, line_ending)), |(id, data, _)| Tile { id, data });
    let mut parsed = map(separated_list1(line_ending, tile), |tiles| ParsedInput { tiles });
    Ok(parsed(input)?)
}

impl Tile {
    fn flips(&self) -> impl Iterator<Item = Tile> {
        Some(self.clone())
            .into_iter()
            .chain(Some({
                let mut tile = self.clone();
                tile.data.invert_axis(arr::Axis(0));
                tile
            }))
            .chain(Some({
                let mut tile = self.clone();
                tile.data.invert_axis(arr::Axis(1));
                tile
            }))
            .chain(Some({
                let mut tile = self.clone();
                tile.data.invert_axis(arr::Axis(0));
                tile.data.invert_axis(arr::Axis(1));
                tile
            }))
    }
    fn reversals(&self) -> impl Iterator<Item = Tile> {
        Some(self.clone()).into_iter().chain(Some(Tile { id: self.id, data: self.data.clone().reversed_axes() }))
    }
    fn positions(&self) -> impl Iterator<Item = Tile> {
        self.reversals().flat_map(|n| n.flips())
    }
    fn matches_top(&self, rhs: &Self) -> bool {
        let rhs_dim = rhs.data.shape()[0];
        self.data.slice(arr::s![0..1, ..]) == rhs.data.slice(arr::s![rhs_dim - 1..rhs_dim, ..])
    }
    fn matches_bottom(&self, rhs: &Self) -> bool {
        let self_dim = self.data.shape()[0];
        self.data.slice(arr::s![self_dim - 1..self_dim, ..]) == rhs.data.slice(arr::s![0..1, ..])
    }
    fn matches_left(&self, rhs: &Self) -> bool {
        let rhs_dim = rhs.data.shape()[1];
        self.data.slice(arr::s![.., 0..1]) == rhs.data.slice(arr::s![.., rhs_dim - 1..rhs_dim])
    }
    fn matches_right(&self, rhs: &Self) -> bool {
        let self_dim = self.data.shape()[1];
        self.data.slice(arr::s![.., self_dim - 1..self_dim]) == rhs.data.slice(arr::s![.., 0..1])
    }
    fn match_dir(&self, rhs: &Self, dir: (isize, isize)) -> bool {
        match dir {
            (-1, 0) => self.matches_left(rhs),
            (0, -1) => self.matches_top(rhs),
            (0, 1) => self.matches_bottom(rhs),
            (1, 0) => self.matches_right(rhs),
            _ => false,
        }
    }
}

pub struct PuzzleSolver {
    tiles: BTreeMap<TileId, Tile>,
    unplaced: Vec<TileId>,
    try_placements: BTreeMap<(isize, isize), Option<Tile>>,
}
impl PuzzleSolver {
    fn from_tiles(inputs: Vec<Tile>) -> PuzzleSolver {
        PuzzleSolver {
            tiles: inputs.clone().into_iter().map(|tile| (tile.id, tile)).collect(),
            unplaced: inputs.into_iter().map(|tile| tile.id).collect(),
            try_placements: BTreeMap::new(),
        }
    }
    fn solved_from_tiles(input: Vec<Tile>) -> PuzzleSolver {
        let mut solver = PuzzleSolver::from_tiles(input);
        solver.place_all();
        solver
    }
    fn get_tile(&self, id: &TileId) -> impl Iterator<Item = Tile> {
        self.tiles.get(id).map(|n| n.positions()).into_iter().flatten()
    }
    fn check_join(&mut self, (y, x): (isize, isize), (dy, dx): (isize, isize)) {
        if let Some(placed_tile) = self.try_placements.clone().get(&(y, x)).cloned().flatten() {
            self.unplaced = self
                .unplaced
                .clone()
                .iter()
                .filter(|&unplaced_tile_id| {
                    for unplaced_tile in self.get_tile(unplaced_tile_id) {
                        let matches = placed_tile.match_dir(&unplaced_tile, (dx, dy));
                        if matches {
                            self.try_placements.insert((y + dy, x + dx), Some(unplaced_tile));
                            return false;
                        }
                    }
                    true
                })
                .copied()
                .collect()
        }
    }
    fn do_placement(&mut self) {
        if self.try_placements.is_empty() {
            self.try_placements.insert((0, 0), self.unplaced.pop().map(|n| self.get_tile(&n).next()).flatten());
        }
        for (&(y, x), _) in self.try_placements.clone().iter().filter_map(|(i, a)| a.as_ref().map(|a| (i, a))) {
            for direction in [(-1, 0), (0, -1), (0, 1), (1, 0)].iter().copied() {
                self.check_join((y, x), direction);
            }
        }
    }
    fn place_all(&mut self) {
        while self.unplaced.len() > 0 {
            self.do_placement();
        }
    }
    fn placed_ids(&self) -> arr::Array2<TileId> {
        let (width, height, offset_x, offset_y) = self.get_metadata();

        let mut arr = arr::Array2::default((width, height));
        for (&(y, x), id) in self.try_placements.iter().filter_map(|(d, v)| v.as_ref().map(|v| (d, v.id))) {
            if let Some(v) = arr.get_mut(((x + offset_x) as usize, (y + offset_y) as usize)) {
                *v = id;
            }
        }
        arr
    }

    fn get_metadata(&self) -> (usize, usize, isize, isize) {
        let ((low_x, high_x), (low_y, high_y)) = self.try_placements.iter().fold(
            ((isize::MAX, isize::MIN), (isize::MAX, isize::MIN)),
            |((low_x, high_x), (low_y, high_y)), (&(y, x), _)| {
                ((low_x.min(x), high_x.max(x)), (low_y.min(y), high_y.max(y)))
            },
        );
        let width = (high_x - low_x) as usize + 1;
        let height = (high_y - low_y) as usize + 1;
        let offset_x = -low_x;
        let offset_y = -low_y;

        (width, height, offset_x, offset_y)
    }

    fn get_picture(&self) -> impl Iterator<Item = Tile> {
        let sample_tile = self.try_placements.get(&(0, 0)).cloned().flatten().unwrap();
        let shape = sample_tile.data.shape();
        let internal_width = shape[1] - 2;
        let internal_height = shape[0] - 2;
        let (external_width, external_height, index_offset_x, index_offset_y) = self.get_metadata();

        let mut new: arr::Array2<bool> =
            arr::Array2::default([external_width * internal_width, external_height * internal_height]);

        for ((y, x), v) in new.indexed_iter_mut() {
            let ex = (x / internal_width) as isize - index_offset_x;
            let ey = (y / internal_height) as isize - index_offset_y;
            let ix = x % internal_width + 1;
            let iy = y % internal_height + 1;

            *v = self.try_placements.get(&(ey, ex)).cloned().flatten().unwrap().data.get((iy, ix)).cloned().unwrap();
        }

        Tile { id: 0, data: new }.positions()
    }
}

fn multiply_corners(tiles: arr::Array2<TileId>) -> usize {
    let shape = tiles.shape();
    let (width, height) = (shape[1], shape[0]);

    tiles.get((0, 0)).unwrap()
        * tiles.get((width - 1, 0)).unwrap()
        * tiles.get((0, height - 1)).unwrap()
        * tiles.get((width - 1, height - 1)).unwrap()
}

const MONSTER_PATTERN: &str = include_str!("monster");
fn monster() -> Tile {
    parse(MONSTER_PATTERN).finish().unwrap().1.tiles[0].clone()
}
impl Tile {
    fn count_monsters(&self) -> usize {
        let self_shape = self.data.shape();
        let (sy, sx) = (self_shape[0], self_shape[1]);
        let monster = monster().data;
        let monster_shape = monster.shape();
        let (my, mx) = (monster_shape[0], monster_shape[1]);
        (0..(sy - my))
            .cartesian_product(0..sx - mx)
            .filter(|&(by, bx)| {
                (0..my).cartesian_product(0..mx).all(|(oy, ox)| {
                    let monster = monster.get((oy, ox)).cloned().unwrap();
                    let this = self.data.get((by + oy, bx + ox)).cloned().unwrap();
                    if monster {
                        this
                    } else {
                        true
                    }
                })
            })
            .count()
    }
    fn count_rough(&self) -> usize {
        self.data.iter().filter(|&n| *n).count()
    }
}

pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let solved = PuzzleSolver::solved_from_tiles(input.tiles);
    Ok(Output {
        task1: multiply_corners(solved.placed_ids()),
        task2: solved
            .get_picture()
            .map(|n| (n.count_monsters(), n.count_rough()))
            .filter(|&(m, _)| m != 0)
            .map(|(m, r)| r - m * monster().count_rough())
            .sum(),
    })
}
