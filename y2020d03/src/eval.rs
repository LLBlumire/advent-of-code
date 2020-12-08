use crate::*;

#[derive(Debug)]
pub struct ParsedInput {
    map: arr::Array2<bool>,
}

pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let empty = map(char('.'), |_| false);
    let tree = map(char('#'), |_| true);
    let location = alt((empty, tree));
    let row = many1(location);
    let world = separated_list1(line_ending, row);
    let world_dim = map(world, |v| {
        let dim1 = v.len();
        let dim2 = v[0].len();
        (v, dim2, dim1)
    });
    let world_arr =
        map_res(world_dim, |(v, x, y)| v.into_iter().flatten().collect::<arr::Array1<_>>().into_shape((y, x)));
    let mut parsed = map(world_arr, |map| ParsedInput { map });
    Ok(parsed(input)?)
}

impl ParsedInput {
    fn get_index(&self, x: usize, y: usize) -> Option<&bool> {
        let (height, width) = {
            let shape = self.map.shape();
            (shape[0], shape[1])
        };
        if y >= height {
            None
        } else {
            self.map.get((y, x % width))
        }
    }

    fn count_slope_true(&self, dx: usize, dy: usize) -> usize {
        (0..)
            .step_by(dx)
            .zip((0..).step_by(dy))
            .map(|(x, y)| self.get_index(x, y).cloned())
            .take_while(Option::is_some)
            .map(Option::unwrap)
            .filter(|n| *n)
            .count()
    }
}
pub type Task1 = usize;
pub type Task2 = usize;
pub fn compute(input: ParsedInput) -> Result<Output> {
    Ok(Output {
        task1: input.count_slope_true(3, 1),
        task2: [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&(dx, dy)| input.count_slope_true(dx, dy))
            .product(),
    })
}
